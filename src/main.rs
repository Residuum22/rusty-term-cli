mod serial;
use std::env;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM5";

fn check_input() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let index = input.trim().parse::<usize>().unwrap();
    index as usize
}

#[tokio::main]
async fn main() {
    let mut args = env::args();
    let _tty_path = args.nth(1).unwrap_or_else(|| DEFAULT_TTY.into());
    let available_baud_rates: Vec<i32> = vec![115_200, 9600, 4800, 2400, 1200, 300];
    
    // Print available serial ports with the index
    let ports: Vec<String> = serial::detect_serial_ports();

    if ports.is_empty() {
        println!("No serial ports found!");
        return;
    }

    println!("Enter the index of the port to connect to: ");
    for (i, port) in ports.iter().enumerate() {
        println!("{}: {}", i, port);
    }
    let tty_index: usize = check_input();
    let tty_path = ports.get(tty_index).unwrap().clone();

    println!("Enter the baud rate: ");
    for (i, baud_rate) in available_baud_rates.iter().enumerate() {
        println!("{}: {}", i, baud_rate);
    }
    let baud_rate_index = check_input();
    let baud_rate = available_baud_rates.get(baud_rate_index as usize).unwrap().clone() as u32;

    let _ = serial::setup_serial(tty_path, baud_rate).await;
    loop {
        // Keep the main thread alive
    }
}