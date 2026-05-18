use clap::Parser;
use std::path::PathBuf;
use stellar_scout_analyzer::{analyze_file, report::ReportFormat};

#[derive(Parser)]
struct Args {
    /// Path to the compiled WASM file to analyze
    wasm: PathBuf,

    /// Output format: json or human
    #[arg(short, long, default_value = "human")]
    format: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let report_format = match args.format.as_str() {
        "json" => ReportFormat::Json,
        "human" => ReportFormat::Human,
        _ => {
            eprintln!("Invalid format: {}. Use 'json' or 'human'.", args.format);
            std::process::exit(1);
        }
    };

    let report = analyze_file(&args.wasm)?;
    let output = report.format(report_format)?;
    println!("{}", output);

    Ok(())
}
