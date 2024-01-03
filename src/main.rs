use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serialport::{self, SerialPort, SerialPortBuilder, SerialPortType, SerialPortInfo};
use clap::Parser;
use anyhow::{Result, bail, Context};
use env_logger;
use log::{info, warn};

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

    #[arg(short, long)]
    reconnect: Option<bool>,
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

fn open_serial_port(args: &Args) -> Result<(SerialPort, String)> {
    let port_name = if args.product_id.is_some() {
        find_by_product_id(&args)?
            .map(|port_info| port_info.port_name)
    } else {
        args.port.clone()
    };

    let port_name = port_name.clone().context("Port not found!")?;
    let baud_rate = args.baud_rate.context("No baud rate specified!")?;
    info!("open port {:?} with rate of {}", port_name, baud_rate);
    let port = SerialPortBuilder::new()
        .baud_rate(baud_rate)
        .open(port_name.clone())?;

    Ok((port, port_name))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    if args.reconnect.unwrap_or_default() {
        let mut retry_count = 0;

        loop {
            let result = open_serial_port(&args);
            match result {
                Ok((port, name)) => {

                    let port_arc = Arc::new(Mutex::new(port.try_clone()?));
                    let port_arc_clone = port_arc.clone();
        
                    // Spawn a thread to read from stdin and write to the serial port.
                    std::thread::spawn(move || {
                        if let Err(_) = read_stdin_loop(port_arc_clone, "port_name_placeholder") {
                            std::process::exit(1);
                        }
                    });
        
                    // Read from serial port and write to stdout in the main thread.
                    match read_serial_loop(port_arc, "port_name_placeholder") {
                        Ok(_) => {
                            // Successful read, break out of the loop
                            break;
                        }
                        Err(_) => {
                            // Reconnect
                                // Delay before attempting the next reconnect
                                std::thread::sleep(Duration::from_secs(1));
        
                                // Decrease the retry count
                                retry_count -= 1;
        
                                // Log a message or take any other necessary action
                                log::warn!("Reconnecting... Retries left: {}", retry_count);
                            
                        }
                    }
                }
                _ => {
                    retry_count += 1;
                    std::thread::sleep(Duration::from_secs(1));
                }
            }

        }
    } else {
        // Connect normally without reconnection logic
        let (port, name): (SerialPort, String) = open_serial_port(&args)?;
        let port_arc = Arc::new(Mutex::new(port));

        let port_arc_clone = port_arc.clone();

        // Spawn a thread to read from stdin and write to the serial port.
        std::thread::spawn(move || {
            if let Err(_) = read_stdin_loop(port_arc_clone, &name) {
                std::process::exit(1);
            }
        });

        // Read from serial port and write to stdout in the main thread.
        match read_serial_loop(port_arc, "port_name_placeholder") {
            Err(_) => {
                // Handle any specific error logic if needed
            }
            _ => {}
        }
    }

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
                warn!("Failed to read from {}: {}", port_name, e);
                bail!("");
            }
        }
    }
}
