use distributed_client::memory::ClientMemory;
use distributed_client::message_dispatch::{create_message_stack, dispatch_messages};
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
    let mut client_memory = Arc::new(Mutex::new(ClientMemory::<String>::new(CircularList::new(
        vec![
            NodeID::from_u16(port),
            NodeID::from_u16(port + 1),
            NodeID::from_u16(port + 2),
        ],
    ))));
    let servers = vec![port, port + 1, port + 2];
    // let servers = initialize_nodes(list_of_servers);

    let message_stack = create_message_stack(9);
    dispatch_messages(message_stack, servers);

    loop {}
}

pub fn initialize_nodes(list_of_servers: Arc<CircularList<u16>>) -> Vec<u16> {
    let mut initialized_nodes: Arc<Mutex<[Option<u16>; 10]>> = Arc::new(Mutex::new([None; 10]));

    <VecDeque<u16> as Clone>::clone(&list_of_servers.elements)
        .into_iter()
        .enumerate()
        .for_each(|(index, port)| {
            let mut initialized_nodes = initialized_nodes.clone();

            let handle = thread::spawn(move || {
                let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
                let message = Message::new(
                    MessageID::new(),
                    Type::Request(Transaction::Init),
                    Some(NodeID::from_u16(port + 1)), // NodeID::from_u16(*list_of_servers.neighbour(list_of_servers.elements)),
                );
                // println!("Message {:#?}", &message);
                let bytes = bincode::serialize(&message).expect("Failed to serialize message");
                stream.write(&bytes);

                // println!("Sent message to {}", port);

                let mut buffer = [0; 1024];
                stream.read(&mut buffer);

                stream.flush();
                initialized_nodes.lock().unwrap()[index] = Some(port);
            });

            handle.join().unwrap();
        });

    let list_of_intialized_servers: Vec<u16> = initialized_nodes
        .lock()
        .unwrap()
        .into_iter()
        .flatten()
        .collect();

    list_of_intialized_servers
}
