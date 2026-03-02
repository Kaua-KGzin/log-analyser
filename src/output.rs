use crate::analyser::{percentage, top_n, LogMetrics};
use crate::cli::Args;
use colored::Colorize;

pub fn print_header() {
    println!("{}", "🔍 Log Analyser".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════".cyan());
    println!();
}

pub fn print_metrics(metrics: &LogMetrics, duration: std::time::Duration) {
    println!("{}", "📊  File Metrics".bright_white().bold());
    println!("{}", "─────────────────────────────".white());
    println!("  Total lines   : {}", metrics.total_lines.to_string().yellow());
    println!("  Parsed lines  : {}", metrics.parsed_lines.to_string().yellow());
    println!("  Analysis time : {:.2?}", duration);
    let lps = metrics.total_lines as f64 / duration.as_secs_f64();
    println!("  Lines/second  : {}", format!("{lps:.0}").bright_green());
    println!();

    println!("{}", "📈  By Log Level".bright_white().bold());
    println!("{}", "─────────────────────────────".white());
    let t = metrics.total_lines;
    println!(
        "  {}    {} ({:.1}%)",
        "INFO   :".green(),
        metrics.info,
        percentage(metrics.info, t)
    );
    println!(
        "  {}    {} ({:.1}%)",
        "WARNING:".yellow(),
        metrics.warnings,
        percentage(metrics.warnings, t)
    );
    println!(
        "  {}    {} ({:.1}%)",
        "ERROR  :".red(),
        metrics.errors,
        percentage(metrics.errors, t)
    );
    println!(
        "  {}    {} ({:.1}%)",
        "DEBUG  :".bright_black(),
        metrics.debug,
        percentage(metrics.debug, t)
    );
    println!();

    if !metrics.pattern_matches.is_empty() {
        println!("{}", "🎯  Detected Patterns".bright_white().bold());
        println!("{}", "─────────────────────────────".white());
        let mut pairs: Vec<_> = metrics.pattern_matches.iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(a.1));
        for (pattern, count) in pairs {
            let label = pattern.replace('_', " ");
            println!("  {:<25} → {} occurrences", label, count.to_string().yellow());
        }
        println!();
    }

    if !metrics.hourly_errors.is_empty() {
        println!("{}", "⏱️   Errors by Hour".bright_white().bold());
        println!("{}", "─────────────────────────────".white());
        let mut hours: Vec<_> = metrics.hourly_errors.iter().collect();
        hours.sort_by_key(|(h, _)| h.clone());
        for (hour, count) in hours {
            let bar = "█".repeat((*count).min(40));
            println!("  {} │ {} {}", hour, bar.red(), count);
        }
        println!();
    }
}

pub fn print_top_ips(metrics: &LogMetrics) {
    if metrics.ip_counts.is_empty() {
        return;
    }
    println!("{}", "🌐  Top IPs".bright_white().bold());
    println!("{}", "─────────────────────────────".white());
    for (ip, count) in top_n(&metrics.ip_counts, 5) {
        println!("  {:<20} {} requests", ip.yellow(), count);
    }
    println!();
}

pub fn print_top_users(metrics: &LogMetrics) {
    if metrics.user_counts.is_empty() {
        return;
    }
    println!("{}", "👤  Top Users".bright_white().bold());
    println!("{}", "─────────────────────────────".white());
    for (user, count) in top_n(&metrics.user_counts, 5) {
        println!("  {:<20} {} events", user.yellow(), count);
    }
    println!();
}

pub fn print_alerts(metrics: &LogMetrics, args: &Args) {
    println!("{}", "🚨  Alerts".bright_white().bold());
    println!("{}", "─────────────────────────────".white());

    let error_rate = percentage(metrics.errors, metrics.total_lines);
    if error_rate >= args.error_threshold {
        println!(
            "  {} Error rate critical: {:.1}% (threshold: {:.0}%)",
            "❌ CRITICAL".red().bold(),
            error_rate,
            args.error_threshold
        );
    } else if error_rate >= args.warn_threshold {
        println!(
            "  {} Error rate elevated: {:.1}% (threshold: {:.0}%)",
            "⚠️  WARNING".yellow().bold(),
            error_rate,
            args.warn_threshold
        );
    } else {
        println!(
            "  {} Error rate nominal: {:.1}%",
            "✅ OK".green().bold(),
            error_rate
        );
    }

    if let Some(&n) = metrics.pattern_matches.get("connection_errors") {
        if n > 2 {
            println!("  {}  Multiple connection failures detected ({})", "⚠️ ".yellow(), n);
        }
    }

    if let Some(&n) = metrics.pattern_matches.get("auth_failures") {
        if n > 0 {
            println!("  {}  Authentication failures detected ({})", "🔐".yellow(), n);
        }
    }

    if let Some(&n) = metrics.pattern_matches.get("critical_crashes") {
        if n > 0 {
            println!("  {} Critical crash signals detected ({})", "💥 CRITICAL".red().bold(), n);
        }
    }

    if let Some(&n) = metrics.pattern_matches.get("resource_spikes") {
        if n > 1 {
            println!("  {}  Repeated resource spikes detected ({})", "📈".yellow(), n);
        }
    }

    println!();
}
