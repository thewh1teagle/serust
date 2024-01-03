use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use serialport::{SerialPort, SerialPortBuilder};
use clap::Parser;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// port name (eg. COM23 OR /dev/ttyUSB0)
    #[arg(short, long)]
    port: Option<String>,

    /// port name (eg. COM23 OR /dev/ttyUSB0)
    #[arg(short, long)]
    baud_rate: Option<u32>,
}

fn do_main() -> Result<(), ()> {
    let args = Args::parse();

    let port = SerialPortBuilder::new().baud_rate(args.baud_rate.unwrap()).open(args.port.clone().unwrap()).unwrap();
    let port = Arc::new(Mutex::new(port));

    // Spawn a thread to read from stdin and write to the serial port.
    let port_clone = args.port.clone();
    std::thread::spawn({
        let port = port.clone();
        move || {
            if let Err(()) = read_stdin_loop(port, &port_clone.unwrap()) {
                std::process::exit(1);
            }
        }
    });

    // Read from serial port and write to stdout in the main thread.
    read_serial_loop(port, &args.port.unwrap())?;

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

fn read_serial_loop(port: Arc<Mutex<SerialPort>>, port_name: &str) -> Result<(), ()> {
    let mut buffer = [0; 512];
    loop {
        let mut port = port.lock().unwrap();
        match port.read(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(n) => {
                std::io::stdout()
                    .write_all(&buffer[..n])
                    .map_err(|e| eprintln!("Error: Failed to write to stdout: {}", e))?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(e) => {
                eprintln!("Error: Failed to read from {}: {}", port_name, e);
                return Err(());
            }
        }
    }
}

fn main() {
    if let Err(()) = do_main() {
        std::process::exit(1);
    }
}
