# 🔍 Log Analyser

> High-performance log file analyser written in Rust — processes millions of lines per second, detects patterns, extracts metrics, and fires configurable alerts.

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen)]()

---

## ✨ Features

- ⚡ **Parallel processing** via [Rayon](https://github.com/rayon-rs/rayon) — uses all CPU cores
- 🧠 **10+ named patterns** — connection errors, auth failures, SQL errors, resource spikes, and more
- 📊 **Rich metrics** — per-level counts, hourly error breakdown, top IPs, top users
- 🚨 **Configurable alerts** — custom error-rate thresholds via CLI flags
- 💾 **JSON export** — pipe results into dashboards or other tools
- 🎨 **Colourised terminal output** — clear at a glance
- 🧪 **Built-in sample generator** — try it without a real log file

---

## 📦 Installation

### Prerequisites

- [Rust 1.75+](https://rustup.rs/)

```bash
# Clone the repository
git clone https://github.com/yourusername/log-analyser.git
cd log-analyser

# Build optimised binary
cargo build --release

# (Optional) install globally
cargo install --path .
```

---

## 🚀 Usage

```bash
# Run with your own log file
cargo run --release -- -f /var/log/app.log

# Generate a sample log and analyse it
cargo run --release -- --sample

# Export results as JSON
cargo run --release -- -f app.log -o report.json

# Custom alert thresholds (warn at 5%, critical at 15%)
cargo run --release -- -f app.log --warn-threshold 5 --error-threshold 15
```

### All flags

```
Usage: log-analyser [OPTIONS]

Options:
  -f, --file <FILE>               Path to the log file to analyse
      --sample                    Generate and analyse a built-in sample log
  -o, --output <FILE>             Export results as JSON to this path
      --error-threshold <FLOAT>   Error-rate % for critical alert  [default: 20]
      --warn-threshold <FLOAT>    Error-rate % for warning alert   [default: 10]
      --top-ips <N>               Show top N IPs                   [default: 5]
      --top-users <N>             Show top N users                 [default: 5]
  -q, --quiet                     Suppress informational output
  -h, --help                      Print help
  -V, --version                   Print version
```

---

## 📊 Example Output

```
🔍 Log Analyser
═══════════════════════════════════════

📊  File Metrics
─────────────────────────────
  Total lines   : 30
  Parsed lines  : 30
  Analysis time : 1.42ms
  Lines/second  : 21126

📈  By Log Level
─────────────────────────────
  INFO   :    13 (43.3%)
  WARNING:     7 (23.3%)
  ERROR  :     9 (30.0%)
  DEBUG  :     4 (13.3%)

🎯  Detected Patterns
─────────────────────────────
  connection errors         → 3 occurrences
  auth failures             → 2 occurrences
  resource spikes           → 2 occurrences
  slow queries              → 1 occurrences
  sql errors                → 1 occurrences

⏱️   Errors by Hour
─────────────────────────────
  2024-02-02 10 │ ███████████ 9

🌐  Top IPs
─────────────────────────────
  192.168.1.100        3 requests
  10.0.0.99            2 requests
  203.0.113.42         1 requests

👤  Top Users
─────────────────────────────
  user123              2 events
  admin                2 events
  alice                1 events

🚨  Alerts
─────────────────────────────
  ❌ CRITICAL  Error rate critical: 30.0% (threshold: 20%)
  ⚠️  Authentication failures detected (2)
  📈  Repeated resource spikes detected (2)
```

---

## 🗂️ Project Structure

```
log-analyser/
├── src/
│   ├── main.rs       # Entry point & CLI wiring
│   ├── cli.rs        # Argument definitions (clap derive)
│   ├── analyser.rs   # Core parsing & parallel aggregation
│   ├── output.rs     # Terminal printing & colourising
│   └── sample.rs     # Built-in sample log generator
├── Cargo.toml
├── .gitignore
├── LICENSE
└── README.md
```

---

## 🧩 Detected Patterns

| Pattern | Description |
|---|---|
| `connection_errors` | Connection lost / failed / timeout / refused |
| `connection_failures` | "Failed to connect" messages |
| `rate_limit_hits` | Rate limiting events |
| `auth_failures` | Authentication / login failures |
| `unauthorized_access` | 401 / 403 / Unauthorized |
| `slow_queries` | Slow database queries |
| `disk_warnings` | Low disk space warnings |
| `resource_spikes` | CPU / memory spikes |
| `critical_crashes` | Panics, null pointers, OOM |
| `sql_errors` | SQL exceptions / errors |

---

## 📐 Log Format

The analyser expects lines in this format (common in most frameworks):

```
YYYY-MM-DD HH:MM:SS LEVEL message...
```

Example:
```
2024-02-02 10:00:45 ERROR Failed to connect to payment gateway: timeout
2024-02-02 10:01:10 WARNING Slow query detected: 2.5s
2024-02-02 10:02:00 INFO  User login: alice
```

Supported levels: `INFO`, `WARNING`, `WARN`, `ERROR`, `CRITICAL`, `DEBUG`

---

## 🛠️ Development

```bash
# Run in debug mode
cargo run -- --sample

# Run tests
cargo test

# Check for warnings
cargo clippy

# Format code
cargo fmt

# Build release binary
cargo build --release
# Binary at: ./target/release/log-analyser
```

---

## 📄 License

MIT © [Kauã Gabriel] [https://github.com/Kaua-KGzin/}
