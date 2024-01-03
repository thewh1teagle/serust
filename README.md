# serust 📡

Serial monitor in Rust

# Features ✨
- Read and write by pressing in terminal
- Auto reconnect
- Connect by port name, and optinally by product ID (USB PID)

# Usage
```console
Usage: serust.exe [OPTIONS]

Options:
  -p, --port <PORT>              port name (eg. COM23 OR /dev/ttyUSB0)
  -b, --baud-rate <BAUD_RATE>    port name (eg. COM23 OR /dev/ttyUSB0) [default: 115200]
      --product-id <PRODUCT_ID>  product ID (as hex value, eg: 000a) find automatically based on pid of usb
  -r, --reconnect <RECONNECT>    [possible values: true, false]
  -h, --help                     Print help
  -V, --version                  Print version
```