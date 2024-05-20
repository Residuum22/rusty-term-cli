use futures::{stream::StreamExt, SinkExt};
use std::{io, str};
use tokio_util::codec::{Decoder, Encoder};
use bytes::{BufMut, BytesMut};
use tokio_serial::SerialPortBuilderExt;
struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }
}

impl Encoder<String> for LineCodec {
    type Error = io::Error;

    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(item.len() + 1);
        dst.put(item.as_bytes());
        Ok(())
    }
}

pub fn detect_serial_ports() -> Vec<String> {
    let ports: Vec<tokio_serial::SerialPortInfo> = tokio_serial::available_ports().expect("No serial ports found!");
    let mut port_names: Vec<String> = Vec::new();
    for port in ports {
        port_names.push(port.port_name);
    }
    port_names
}

pub async fn setup_serial(tty_path: String, baud_rate: u32) -> tokio_serial::Result<()> {
    let port = tokio_serial::new(tty_path, baud_rate).open_native_async()?;

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let stream = LineCodec.framed(port);
    let (mut tx, mut rx) = stream.split();

    tokio::spawn(async move {
        loop {
            let item = rx
                .next()
                .await
                .expect("Error awaiting future in RX stream.")
                .expect("Reading stream resulted in an error");
            print!("{item}");
        }
    });

    tokio::spawn(async move {
        loop {
            // Get input from stdin and send it to the serial port
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let write_result = tx
                .send(String::from(format!("{input}")))
                .await;
            match write_result {
                Ok(_) => (),
                Err(err) => println!("{:?}", err),
            }
        }
    });
    Ok(())

}