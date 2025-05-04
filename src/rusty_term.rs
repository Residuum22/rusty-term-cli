use clap::error;
use log;
use log::error;
use serialport;
use std::io::{BufRead, ErrorKind, Stdout};
use std::io::{BufWriter, Read, Write};
use std::{result, thread};
use std::sync::{Arc, Mutex};

const DEFAULT_BUFFER_SIZE: usize = 1024;

pub fn print_serial_ports() {
    let ports = serialport::available_ports().expect("No ports found!");
    for port in ports {
        println!("Port: {}", port.port_name);
    }
}

fn open_port(port_name: String, baud_rate: u32) -> Box<dyn serialport::SerialPort> {
    let port = serialport::new(port_name, baud_rate)
        .open()
        .expect("Failed to open serial port!");
    port
}

fn write_to_stdout(stdout: &mut BufWriter<Stdout>, buffer: &[u8], n: usize) {
    stdout.write_all(&buffer[..n]).unwrap();
    stdout.flush().unwrap();
}

fn read_serial_port_loop(port: Arc<Mutex<Box<dyn serialport::SerialPort>>>) {
    let mut buffer = [0u8; DEFAULT_BUFFER_SIZE];
    loop {
        let mut ported = port.lock().unwrap(); // Lock the port for reading
        let result = ported.read(&mut buffer);
        match result {
            Ok(n) => {write_to_stdout(&mut BufWriter::new(std::io::stdout()), &buffer, n)},
            Err(e) if e.kind() == ErrorKind::TimedOut => continue,
            Err(e) => {
                error!("ErrorKind: {} | ErrorMessage: {}", e, e);
                return;
            }
        }
    }
}

pub fn run_rusty_term(port_name: String, baud_rate: u32) -> Result<String, String> {
    let port = open_port(port_name, baud_rate);
    let port_lock = Arc::new(Mutex::new(port));
    let port_lock_clone = Arc::clone(&port_lock);
    let mut stdin = std::io::stdin().lock();

    let mut buffer_stdin = [0u8; 1024];

    thread::spawn(|| {
        read_serial_port_loop(port_lock_clone);
    });

    loop {
        let read = stdin.read(&mut buffer_stdin).unwrap();
        if read == 0 {
            continue;
        }
        let mut port_lock = port_lock.lock().unwrap(); // Lock the port for writing
        port_lock.write(&mut buffer_stdin[..read]).unwrap();
    }
}
