use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, io};

use models::message::{Response, Type};
use models::tcp_client::{Stream, TcpClient};
use models::{
    message::{Message, Transaction},
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

    // let mut buffer = [0; 1024];
    let mut buffer: Vec<u8> = vec![];

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Could not unwrap TcpStream",
            ));
        };
        
        stream.read(&mut buffer);

        println!("Received message for {}", port);
        let Ok(message) = bincode::deserialize::<Message>(&buffer) else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Could not deserialize message",
            ));
        };

        println!("Message: {:#?}", message.clone());

        // Split into new function
        match message.message_type {
            Type::Request(transaction) => {
                let node = handle_transactions(
                    transaction,
                    port,
                    message.neighbour.expect("No neighbour"),
                );
                // println!("{:#?}", node);
                let Ok(byte_node) = bincode::serialize(&node) else {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Could not serialize Node",
                    ));
                };
                // stream.write(buffer, port);
                stream.flush();
            }
            Type::Response(response) => handle_response(response),
            Type::Chunk(ref transaction_chunk) => {
                let neighbour = message.clone().neighbour.expect("No neighbour");
                let _ = transaction_chunk.iter().map(|transaction| {
                    handle_transactions(
                        transaction.clone(),
                        port,
                        message.clone().neighbour.expect("No neighbour"),
                    )
                });
            }
        }
    }
    Ok(())
}

pub fn handle_transactions(transaction: Transaction, port: u16, neighbour: NodeID) -> Response {
    match transaction {
        Transaction::Init => {
            println!("Initialized node: {}", port);
            Response::InitOk(Node::new(NodeID::from(port.to_string()), neighbour))
        }
        Transaction::Gossip(_) => todo!(),
        Transaction::Delete(_) => todo!(),
        Transaction::Insert(message) => {
            println!("{:?}", &message);
            Response::InsertOk(Node::new(NodeID::from(port.to_string()), neighbour))
        }
        Transaction::Fetch(_) => todo!(),
    }
}

pub fn handle_response(response: Response) {
    match response {
        Response::InitOk(_) => {}
        Response::InsertOk(_) => todo!(),
    }
}
