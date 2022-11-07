use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use netlify_ddns::{run, Args};

fn main() -> Result<()> {
    // Initialize the tracing-based logs
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let args = Args::parse();

    run(args)
}
