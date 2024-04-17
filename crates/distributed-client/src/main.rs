use models::messages::{self, Init, Message, MessageID, Transaction};
use serde::Serialize;
use std::env;
use std::io::Write;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let port: u16 = args
        .get(1)
        .expect("Failed to get port argument")
        .parse()
        .expect("Failed to parse arg as u16");

    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    
    let message = Message::new(MessageID::new(), Transaction::Init(Init {}));
    let bytes = bincode::serialize(&message).expect("Failed to serialize message");
    stream.write(&bytes);
    Ok(())
}
