// src/main.rs
/*
 * Main executable for DeFiTestnetAICoreNext
 */

use clap::Parser;
use defitestnetaicorenext::{Result, run};

#[derive(Parser)]
#[command(version, about = "DeFiTestnetAICoreNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
