use models::messages::{self, Init, Message, MessageID, Transaction, Type};
use models::node::Node;
use serde::Serialize;
use std::{env, thread};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let port: u16 = args
        .get(1)
        .expect("Failed to get port argument")
        .parse()
        .expect("Failed to parse arg as u16");

    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    thread::spawn(move ||  {
        let mut buffer = [0; 1024];
        let message = Message::new(MessageID::new(), Type::Request(Transaction::Init));
        let bytes = bincode::serialize(&message).expect("Failed to serialize message");
        stream.write(&bytes);
        stream.read(&mut buffer);
        println!("{:#?}", bincode::deserialize::<Node>(&buffer));
    });
    loop {}
    Ok(())
}
