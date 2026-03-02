use clap::Parser;

/// 🔍 High-performance real-time log analyser
#[derive(Parser, Debug)]
#[command(
    name = "log-analyser",
    version,
    about = "Analyses log files, extracts metrics and fires alerts",
    long_about = None
)]
pub struct Args {
    /// Path to the log file to analyse
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<String>,

    /// Generate and analyse a sample log file
    #[arg(long, default_value_t = false)]
    pub sample: bool,

    /// Export results as JSON to this path
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// Error-rate threshold (%) that triggers a critical alert  [default: 20]
    #[arg(long, default_value_t = 20.0)]
    pub error_threshold: f64,

    /// Warn threshold (%) that triggers a warning alert         [default: 10]
    #[arg(long, default_value_t = 10.0)]
    pub warn_threshold: f64,

    /// Show top N IPs                                           [default: 5]
    #[arg(long, default_value_t = 5)]
    pub top_ips: usize,

    /// Show top N users                                         [default: 5]
    #[arg(long, default_value_t = 5)]
    pub top_users: usize,

    /// Silence informational output (errors/alerts still shown)
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,
}
