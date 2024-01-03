use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use serialport::{self, SerialPort, SerialPortBuilder, SerialPortType, SerialPortInfo};
use clap::Parser;
use anyhow::{Result, bail, Context};
use env_logger;
use log::{debug, info};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// port name (eg. COM23 OR /dev/ttyUSB0)
    #[arg(short, long)]
    port: Option<String>,

    /// port name (eg. COM23 OR /dev/ttyUSB0)
    #[arg(short, long, default_value = "115200")]
    baud_rate: Option<u32>,

    /// product ID (as hex value, eg: 000a)
    /// find automatically based on pid of usb
    #[arg(long)]
    product_id: Option<String>, 
}

fn find_by_product_id(args: &Args) -> Result<Option<SerialPortInfo>> {
    let ports = serialport::available_ports().unwrap();
    for port in ports {
        let port_clone = port.clone();
        match port.port_type {
            SerialPortType::UsbPort(info) => {
                let pid = format!("{:04x}", info.pid);
                if args.product_id.clone().unwrap() == pid {
                    return Ok(Some(port_clone))
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

fn open_serial_port(args: &Args) -> Result<SerialPort> {
    let port_name = if args.product_id.is_some() {
        find_by_product_id(&args)?
            .map(|port_info| port_info.port_name)
    } else {
        args.port.clone()
    };

    info!("open port {:?} with rate of {}", port_name, args.baud_rate.unwrap());
    let port = SerialPortBuilder::new()
        .baud_rate(args.baud_rate.unwrap())
        .open(port_name.unwrap())?;

    Ok(port)
}


fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    let port = open_serial_port(&args)?;
    let port_arc = Arc::new(Mutex::new(port));

    let port_arc_clone = port_arc.clone();

    // Spawn a thread to read from stdin and write to the serial port.
    std::thread::spawn(move || {
        if let Err(_) = read_stdin_loop(port_arc_clone, "port_name_placeholder") {
            std::process::exit(1);
        }
    });

    // Read from serial port and write to stdout in the main thread.
    read_serial_loop(port_arc, "port_name_placeholder")?;

    Ok(())
}

fn read_stdin_loop(port: Arc<Mutex<SerialPort>>, port_name: &str) -> Result<(), ()> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = [0; 512];
    loop {
        let read = stdin
            .read(&mut buffer)
            .map_err(|e| eprintln!("Error: Failed to read from stdin: {}", e))?;
        if read == 0 {
            return Ok(());
        } else {
            let mut port = port.lock().unwrap();
            port.write(&buffer[..read])
                .map_err(|e| eprintln!("Error: Failed to write to {}: {}", port_name, e))?;
        }
    }
}

fn read_serial_loop(port: Arc<Mutex<SerialPort>>, port_name: &str) -> Result<()> {
    let mut buffer = [0; 512];
    loop {
        let mut port = port.lock().unwrap();
        match port.read(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(n) => {
                std::io::stdout()
                    .write_all(&buffer[..n])
                    .context("Failed to write to stdout")?
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(e) => {
                eprintln!("Error: Failed to read from {}: {}", port_name, e);
                bail!("");
            }
        }
    }
}
