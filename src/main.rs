use anyhow::Result;
use clap::Parser;

use netlify_ddns::{run, Args};

fn main() -> Result<()> {
    pretty_env_logger::init_timed();
    let args = Args::parse();

    run(args)
}
