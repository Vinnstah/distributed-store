use std::{env, io};
use std::io::{Read, Write};
use std::net::TcpListener;

use models::{
    messages::{Message, Transaction},
    node::{Node, NodeID},
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let port: u16 = args
        .get(1)
        .expect("Failed to get port argument")
        .parse()
        .expect("Failed to parse arg as u16");

    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();

    let mut buffer = [0; 1024];

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Could not unwrap TcpStream"))
        };
        
        stream.read(&mut buffer);

        println!("{:#?}", bincode::deserialize::<Message>(&buffer));
        let Ok(message) = bincode::deserialize::<Message>(&buffer) else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Could not deserialize message"))
        };

        match message.message_type {
            models::messages::Type::Request(request) => {
                let node = handle_transactions(request);
                println!("{:#?}", node);
                let Ok(byte_node) = bincode::serialize(&node) else {
                    return Err(io::Error::new(io::ErrorKind::Other, "Could not serialize Node"))
                };
                stream.write(&byte_node);
            }
            models::messages::Type::Response(response) => todo!(),
        }

    }
    Ok(())
}

pub fn handle_transactions(transaction: Transaction) -> Node {
    match transaction {
        Transaction::Init => Node::new(NodeID::new(), NodeID::from("id2")),
        Transaction::Gossip(_) => todo!(),
        Transaction::Delete(_) => todo!(),
        Transaction::Insert(_) => todo!(),
    }
}
