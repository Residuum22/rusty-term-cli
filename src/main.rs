use clap::{ArgAction, Parser};
use log::{info, LevelFilter};
use simple_logger;
mod rusty_term;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, action=ArgAction::SetTrue, default_value = "false")]
    pub list: bool,

    #[arg(short, long, default_value = "DUMMY")]
    pub port: String,

    #[arg(short, long, default_value = "115200")]
    #[arg(value_parser = clap::value_parser!(u32).range(50..=921600))]
    pub baud_rate: u32,
}

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .without_timestamps()
        .init()
        .unwrap();
    let args = Args::parse();
    if args.list {
        info!("Listing serial ports...");
        rusty_term::print_serial_ports();
    } else {
        info!("Opening serial port: {}", args.port);
        info!("Baud rate: {}", args.baud_rate);
        let _ = rusty_term::run_rusty_term(args.port, args.baud_rate);
    }
}
