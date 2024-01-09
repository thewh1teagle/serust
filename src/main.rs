mod actions;
mod args;
mod serial;
use anyhow::Result;
use args::Args;
use clap::Parser;
use env_logger;

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    args.validate()?;
    if args.list {
        actions::list_ports()?;
    } else if args.reconnect.unwrap_or_default() {
        serial::open_with_reconnect(&args)?;
    } else {
        serial::open(&args)?;
    }
    Ok(())
}
