use log;
use log::error;
use serialport;
use std::io::{ErrorKind, Stdout};
use std::io::{BufWriter, Read, Write};
use std::thread;

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

fn write_to_stdout(stdout: &mut BufWriter<Stdout>, buffer: &[u8], n: usize) {
    stdout.write_all(&buffer[..n]).unwrap();
    stdout.flush().unwrap();
}

fn serial_xd(port: &mut dyn serialport::SerialPort, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
    loop {
        let n = port.read(buffer)?;
        if n > 0 {
            write_to_stdout(&mut BufWriter::new(std::io::stdout()), buffer, n);
        }
    }
}

pub fn run_rusty_term(port_name: String, baud_rate: u32) -> Result<String, String> {
    let mut buffer = [0u8; DEFAULT_BUFFER_SIZE];
    let mut port = open_port(port_name, baud_rate);
    let mut stdout = BufWriter::new(std::io::stdout());

    thread::spawn(f)

    loop {
        let result = port.read(&mut buffer);
        match result {
            Ok(n) => {
                write_to_stdout(&mut stdout, &buffer, n);
            }
            Err(e) if e.kind() == ErrorKind::TimedOut => continue,
            Err(e) => {
                error!("ErrorKind: {} | ErrorMessage: {}", e, e);
                return Err("Error in run_rusty_term".to_string());
            }
        }
    }
}
