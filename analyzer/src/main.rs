use clap::Parser;
use std::path::PathBuf;
use wasmparser::{Parser as WpParser, Payload};

#[derive(Parser)]
struct Args {
    /// Path to the compiled WASM file to analyze
    wasm: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let data = std::fs::read(&args.wasm)?;
    let mut parser = WpParser::new(0);
    for payload in parser.parse_all(&data) {
        match payload? {
            Payload::Version { .. } => println!("WASM version section found"),
            Payload::CodeSectionEntry(_) => println!("Found code section entry"),
            _ => {}
        }
    }
    println!("Analysis complete: (placeholder) no issues found.");
    Ok(())
}
