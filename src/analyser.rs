use crate::cli::Args;
use anyhow::Result;
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// ── Data structures ──────────────────────────────────────────────────────────

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LogMetrics {
    pub total_lines:      usize,
    pub parsed_lines:     usize,
    pub errors:           usize,
    pub warnings:         usize,
    pub info:             usize,
    pub debug:            usize,
    pub unknown:          usize,
    pub pattern_matches:  HashMap<String, usize>,
    pub ip_counts:        HashMap<String, usize>,
    pub user_counts:      HashMap<String, usize>,
    pub hourly_errors:    HashMap<String, usize>,
}

// ── Named patterns ────────────────────────────────────────────────────────────

struct NamedPattern {
    regex: Regex,
    name:  &'static str,
}

fn build_patterns() -> Vec<NamedPattern> {
    let raw: &[(&str, &str)] = &[
        (r"(?i)connection.*(lost|failed|timeout|refused)", "connection_errors"),
        (r"(?i)failed to connect",                         "connection_failures"),
        (r"(?i)rate.?limit",                               "rate_limit_hits"),
        (r"(?i)authentication? failed",                    "auth_failures"),
        (r"(?i)unauthorized|403|401",                      "unauthorized_access"),
        (r"(?i)slow.?quer|query.*\d+\.\d+s",               "slow_queries"),
        (r"(?i)(disk|storage).*(low|full|\d+%)",           "disk_warnings"),
        (r"(?i)(cpu|memory|ram).*(high|spike|\d{2,3}%)",   "resource_spikes"),
        (r"(?i)null.?pointer|panic|segfault|OOM",          "critical_crashes"),
        (r"(?i)sql.*(error|exception|fail)",                "sql_errors"),
    ];

    raw.iter()
        .filter_map(|(re, name)| {
            Regex::new(re).ok().map(|regex| NamedPattern { regex, name })
        })
        .collect()
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn analyse_log(filename: &str, _args: &Args) -> Result<LogMetrics> {
    let file   = File::open(filename)?;
    let reader = BufReader::new(file);

    // Read all lines up-front so we can parallelise with Rayon
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

    let log_re = Regex::new(
        r"(?P<ts>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})\s+(?P<lvl>INFO|WARNING|WARN|ERROR|DEBUG|CRITICAL)\s+(?P<msg>.+)",
    )?;
    let ip_re   = Regex::new(r"\b(\d{1,3}(?:\.\d{1,3}){3})\b")?;
    let user_re = Regex::new(r"(?i)user[:\s]+([a-zA-Z0-9_\-\.@]+)")?;
    let patterns = build_patterns();

    // ── Parallel aggregation ─────────────────────────────────────────────────
    // Each thread processes a chunk and returns a partial LogMetrics.
    // We then fold everything into a single result.

    let metrics = lines
        .par_iter()
        .map(|line| {
            let mut m = LogMetrics::default();
            m.total_lines = 1;

            if let Some(caps) = log_re.captures(line) {
                m.parsed_lines = 1;

                let lvl = caps.name("lvl").map_or("", |c| c.as_str());
                let msg = caps.name("msg").map_or("", |c| c.as_str());
                let ts  = caps.name("ts").map_or("", |c| c.as_str());

                match lvl {
                    "ERROR" | "CRITICAL" => m.errors   += 1,
                    "WARNING" | "WARN"   => m.warnings  += 1,
                    "INFO"               => m.info      += 1,
                    "DEBUG"              => m.debug     += 1,
                    _                    => m.unknown   += 1,
                }

                // Hourly error buckets
                if matches!(lvl, "ERROR" | "CRITICAL") {
                    let hour = ts.get(..13).unwrap_or(ts).to_string(); // "YYYY-MM-DD HH"
                    *m.hourly_errors.entry(hour).or_insert(0) += 1;
                }

                // Named pattern matches
                for np in &patterns {
                    if np.regex.is_match(msg) {
                        *m.pattern_matches.entry(np.name.to_string()).or_insert(0) += 1;
                    }
                }

                // IP extraction
                for cap in ip_re.captures_iter(line) {
                    let ip = cap.get(1).map_or("", |c| c.as_str()).to_string();
                    *m.ip_counts.entry(ip).or_insert(0) += 1;
                }

                // User extraction
                if let Some(cap) = user_re.captures(line) {
                    let user = cap.get(1).map_or("", |c| c.as_str()).to_string();
                    *m.user_counts.entry(user).or_insert(0) += 1;
                }
            } else {
                m.unknown = 1;
            }

            m
        })
        .reduce(LogMetrics::default, |mut acc, partial| {
            acc.total_lines  += partial.total_lines;
            acc.parsed_lines += partial.parsed_lines;
            acc.errors       += partial.errors;
            acc.warnings     += partial.warnings;
            acc.info         += partial.info;
            acc.debug        += partial.debug;
            acc.unknown      += partial.unknown;

            for (k, v) in partial.pattern_matches { *acc.pattern_matches.entry(k).or_insert(0) += v; }
            for (k, v) in partial.ip_counts       { *acc.ip_counts.entry(k).or_insert(0)       += v; }
            for (k, v) in partial.user_counts     { *acc.user_counts.entry(k).or_insert(0)     += v; }
            for (k, v) in partial.hourly_errors   { *acc.hourly_errors.entry(k).or_insert(0)   += v; }

            acc
        });

    Ok(metrics)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

pub fn percentage(part: usize, total: usize) -> f64 {
    if total == 0 { 0.0 } else { part as f64 / total as f64 * 100.0 }
}

/// Return top-N entries from a map sorted by count descending.
pub fn top_n(map: &HashMap<String, usize>, n: usize) -> Vec<(&String, &usize)> {
    let mut pairs: Vec<_> = map.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1));
    pairs.truncate(n);
    pairs
}
