use anyhow::Result;
use serialport5::{self, SerialPortType};

pub fn list_ports() -> Result<()> {
    let ports = serialport5::available_ports()?;

    match ports.len() {
        0 => println!("No ports found."),
        1 => println!("Found 1 port:"),
        n => println!("Found {} ports:", n),
    };

    for (index, port) in ports.into_iter().enumerate() {
        println!("{}. {}", index + 1, port.port_name);

        match port.port_type {
            SerialPortType::UsbPort(info) => {
                println!("   Type: USB");
                println!("   VID: {:04x} PID: {:04x}", info.vid, info.pid);
                println!(
                    "   Serial Number: {}",
                    info.serial_number.as_deref().unwrap_or("N/A")
                );
                println!(
                    "   Manufacturer: {}",
                    info.manufacturer.as_deref().unwrap_or("N/A")
                );
            }
            _ => {
                println!("   Type: Unknown");
            }
        }
    }

    Ok(())
}
