use std::io;

#[derive(Debug)]
pub struct Args {
    pub domain: String,
    pub subdomain: String,
    pub ipv6: bool,
    pub token: String,
}

pub fn run(args: Args) -> Result<(), io::Error> {
    Ok(())
}
