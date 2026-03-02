mod analyser;
mod cli;
mod output;
mod sample;

use anyhow::Result;
use cli::Args;
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();

    let log_path = if args.sample {
        let path = "sample.log";
        sample::create_sample_log(path)?;
        println!("{}", colored::Colorize::bright_cyan("📝 Sample log file created: sample.log\n"));
        path.to_string()
    } else {
        match &args.file {
            Some(f) => f.clone(),
            None => {
                eprintln!("❌ Provide a log file with -f <FILE> or use --sample to generate one.");
                std::process::exit(1);
            }
        }
    };

    output::print_header();

    let start = std::time::Instant::now();
    let metrics = analyser::analyse_log(&log_path, &args)?;
    let duration = start.elapsed();

    output::print_metrics(&metrics, duration);
    output::print_top_ips(&metrics);
    output::print_top_users(&metrics);
    output::print_alerts(&metrics, &args);

    if let Some(out) = &args.output {
        let json = serde_json::to_string_pretty(&metrics)?;
        std::fs::write(out, json)?;
        println!("\n💾 Report saved to {out}");
    }

    Ok(())
}
