use log;
use log::error;
use serialport;
use std::io::ErrorKind;
use std::io::{BufWriter, Read, Write};

const DEFAULT_BUFFER_SIZE: usize = 1024;

pub fn print_serial_ports() {
    let ports = serialport::available_ports().expect("No ports found!");
    for port in ports {
        println!("Port: {}", port.port_name);
    }
}

fn open_port(port_name: String, baud_rate: u32) -> Box<dyn serialport::SerialPort> {
    let port = serialport::new(port_name, baud_rate)
        .timeout(std::time::Duration::from_secs(5))
        .open()
        .expect("Failed to open serial port!");
    port
}

pub fn run_rusty_term(port_name: String, baud_rate: u32) -> Result<String, String> {
    let mut buffer = [0u8; DEFAULT_BUFFER_SIZE];
    let mut port = open_port(port_name, baud_rate);
    let mut stdout = BufWriter::new(std::io::stdout());

    loop {
        let result = port.read(&mut buffer);
        match result {
            Ok(n) => {
                stdout.write_all(&buffer[..n]).unwrap();
                stdout.flush().unwrap();
            }
            Err(e) if e.kind() == ErrorKind::TimedOut => continue,
            Err(e) => {
                error!("ErrorKind: {} | ErrorMessage: {}", e, e);
                return Err("Error in run_rusty_term".to_string());
            }
        }
    }
}
