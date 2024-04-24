use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};

use models::{
    message::{CircularList, Message, MessageID, Transaction, Type},
    node::NodeID,
};

use crate::message_dispatch::Client;

impl Client {
    pub fn initialize_nodes(list_of_servers: Arc<CircularList<u16>>) -> Vec<u16> {
        let mut initialized_nodes: Arc<Mutex<[Option<u16>; 10]>> = Arc::new(Mutex::new([None; 10]));

        <Vec<u16> as Clone>::clone(&list_of_servers.elements)
            .into_iter()
            .enumerate()
            .for_each(|(index, port)| {
                let mut initialized_nodes = initialized_nodes.clone();

                let handle = thread::spawn(move || {
                    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
                    let message = Message::new(
                        MessageID::new(),
                        Type::Request(Transaction::Init),
                        Some(NodeID::from_u16(port + 1)),
                    );
                    let bytes = bincode::serialize(&message).expect("Failed to serialize message");
                    stream.write(&bytes);

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
}
