use std::{
    io::{self, Read, Write},
    net::TcpStream,
    thread,
};

use std::io::Error;

use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Stream {
    pub buffer: Vec<u8>
}

impl Stream {
    pub fn new() -> Self {
        Self { buffer: vec![] }
    }
}

pub trait TcpClient {
    fn write(&self, buffer: Vec<u8>, port: u16) -> io::Result<()>;
    fn read(&self, buffer: &'static mut Vec<u8>, port: u16) -> Result<Message, Error>;
}

impl TcpClient for Stream {
    fn write(&self, buffer: Vec<u8>, port: u16) -> io::Result<()> {
        thread::spawn(move || {
            let mut stream: TcpStream = TcpStream::connect(("127.0.0.1", port)).unwrap();
            let _ = stream.write_all(&buffer);
            stream.flush()
        })
        .join()
        .expect("Failed to join thread")
    }

    fn read(&self, buffer: &'static mut Vec<u8>, port: u16) -> Result<Message, Error> {
        thread::spawn(move || {
            let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
            stream.read_to_end(buffer);

            let Ok(message) = bincode::deserialize::<Message>(&buffer) else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Could not deserialize message",
                ));
            };
            return Ok(message);
        })
        .join()
        .expect("Failed to join thread")
    }
}
