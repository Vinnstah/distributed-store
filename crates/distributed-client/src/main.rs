use distributed_client::memory::ClientMemory;
use models::message::{CircularList, Message, MessageID, Transaction, Type};
use models::node::NodeID;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::{env, thread};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let port: u16 = args
        .get(1)
        .expect("Failed to get port argument")
        .parse()
        .expect("Failed to parse arg as u16");

    let list_of_servers = Arc::new(CircularList::new(vec![port, port + 1, port + 2]));

    <VecDeque<u16> as Clone>::clone(&list_of_servers.elements)
        .into_iter()
        .for_each(|port| {
            let handle = thread::spawn(move || {
                let mut client_memory = Arc::new(Mutex::new(ClientMemory::<String>::new(
                    CircularList::new(vec![
                        NodeID::from_u16(port),
                        NodeID::from_u16(port + 1),
                        NodeID::from_u16(port + 2),
                    ]),
                )));
                let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
                let message = Message::new(
                    MessageID::new(),
                    Type::Request(Transaction::Init),
                    NodeID::from_u16(port + 1), // NodeID::from_u16(*list_of_servers.neighbour(list_of_servers.elements)),
                );

                let bytes = bincode::serialize(&message).expect("Failed to serialize message");
                stream.write(&bytes);

                println!("Sent message to {}", port);

                let mut buffer = [0; 1024];
                stream.try_clone().unwrap().read(&mut buffer);

                stream.flush();

                let message = Message::new(
                    MessageID::new(),
                    Type::Request(Transaction::Init),
                    NodeID::from_u16(port + 1), // NodeID::from_u16(*list_of_servers.neighbour(list_of_servers.elements)),
                );

                client_memory
                    .lock()
                    .unwrap()
                    .insert_value_for_nodes("2".to_string(), vec![]);

                println!("{:#?}", &client_memory);
            });

            handle.join().unwrap();
        });

    // println!("{:#?}", bincode::deserialize::<Node>(&buffer));

    loop {}
}

pub fn initialize_nodes(list_of_servers: CircularList<NodeID>) {}

pub fn dispatch_messages(amount: u16) {
    
}
