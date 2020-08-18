#![forbid(unsafe_code)]
#![forbid(warnings)]

use anyhow::Result;
use structopt::StructOpt;

use netlify_ddns::{run, Args};

fn main() -> Result<()> {
    pretty_env_logger::init_timed();
    let args = Args::from_args();

    run(args)
}
