use distributed_client::memory::ClientMemory;
use models::message::{CircularList, Message, MessageID, Transaction, Type};
use models::node::{Node, NodeID};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Mutex;
use std::{env, thread};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let port: u16 = args
        .get(1)
        .expect("Failed to get port argument")
        .parse()
        .expect("Failed to parse arg as u16");

    let list_of_servers = CircularList::new(vec![port, port + 1, port + 2]);
    let mut client_memory = Mutex::new(ClientMemory::<String>::new(CircularList::new(vec![
        NodeID::from_u16(port),
        NodeID::from_u16(port + 1),
        NodeID::from_u16(port + 2),
    ])));
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    thread::spawn(move || {
        let message = Message::new(
            MessageID::new(),
            Type::Request(Transaction::Init),
            NodeID::from_u16(*list_of_servers.neighbour(0)),
        );

        let bytes = bincode::serialize(&message).expect("Failed to serialize message");
        stream.write(&bytes);

        client_memory
            .get_mut()
            .unwrap()
            .insert_value_for_nodes("2".to_string(), vec![message.neighbour]);
        let mut buffer = [0; 1024];
        stream.read(&mut buffer);
        // println!("{:#?}", bincode::deserialize::<Node>(&buffer));
        println!("{:#?}", &client_memory);
    });
    loop {}
}

pub fn initialize_nodes() {}
