use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// port name (eg. COM23 OR /dev/ttyUSB0)
    #[arg(short, long)]
    pub port: Option<String>,

    /// port name (eg. COM23 OR /dev/ttyUSB0)
    #[arg(short, long, default_value = "115200")]
    pub baud_rate: Option<u32>,

    /// product ID (as hex value, eg: 000a)
    /// find automatically based on pid of usb
    #[arg(long)]
    pub product_id: Option<String>,

    /// reconnect automatically if disconnected
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub reconnect: Option<bool>,

    /// List available ports
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub list: Option<bool>,
}
