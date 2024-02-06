use anyhow::{bail, Result};
use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// port name (eg. COM23 OR /dev/ttyUSB0)
    #[arg(short, long)]
    pub port: Option<String>,

    /// product ID (as hex value, eg: 000a)
    /// find automatically based on pid of usb
    #[arg(long)]
    pub product_id: Option<String>,

    /// vendor ID (as hex value, eg: 000a)
    /// find automatically based on pid of usb
    #[arg(long)]
    pub vendor_id: Option<String>,

    /// baud rate
    #[arg(short, long, default_value = "115200")]
    pub baud_rate: Option<u32>,

    /// reconnect automatically if disconnected
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub reconnect: Option<bool>,

    /// List available ports
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub list: bool,

    /// Buffer size of stdout
    #[arg(short, long)]
    pub buf_size: Option<usize>,

    /// flush stdout after every write
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub flush: bool,
}

impl Args {
    pub fn validate(&self) -> Result<()> {
        if self.list {
            // If list is true, no other arguments are required
            Ok(())
        } else if self.port.is_none() && self.vendor_id.is_none() && self.product_id.is_none() {
            // If list is not provided, at least one of port, vendor_id, or product_id must be provided
            bail!("Please provide either a port, vendor ID, or product ID")
        } else {
            Ok(())
        }
    }
}
