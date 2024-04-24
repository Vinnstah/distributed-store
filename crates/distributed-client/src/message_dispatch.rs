use std::{
    borrow::Borrow,
    io::{self, Read, Write},
    net::TcpStream,
    ops::Rem,
    slice::Chunks,
};

use models::{
    message::{CircularIterator, CircularList, Insert, Message, MessageID, Transaction, Type},
    node::NodeID,
};
use rand::random;

pub struct Client {
    stream: dyn TcpClient,
}

pub trait TcpClient {
    fn write(&self, buffer: Vec<u8>, port: u16) -> io::Result<()>;
    fn read(&self, buffer: &mut Vec<u8>, port: u16) -> io::Result<usize>;
}

impl TcpClient for Client {
    fn write(&self, buffer: Vec<u8>, port: u16) -> io::Result<()> {
        let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
        let _ = stream.write_all(&buffer);
        stream.flush()
    }

    fn read(&self, buffer: &mut Vec<u8>, port: u16) -> io::Result<usize> {
        let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
        stream.read_to_end(buffer)
    }
}

impl Client {
    pub fn create_message_stack(amount: usize) -> Vec<Transaction> {
        let mut stack_of_messages: Vec<Transaction> = vec![];
        for _ in 0..amount {
            stack_of_messages.push(Transaction::Insert(Insert::new(
                MessageID::new(),
                random::<u16>(),
            )));
        }
        println!("Create stack of {} messages", amount);
        stack_of_messages
    }

    pub fn dispatch_messages(messages: Vec<Transaction>, servers: Vec<u16>) {
        let chunked_messages = messages.chunks(10);
        println!("Create {} chunks", chunked_messages.len());

        chunked_messages
            .into_iter()
            .enumerate()
            .for_each(|(index, chunk)| {
                let server_index = (index + 1) % servers.len();
                Self::dispatch_chunk(
                    chunk,
                    &servers[server_index],
                    servers[(index + 2) % servers.len()],
                );
            });
    }

    pub fn dispatch_chunk(chunk: &[Transaction], server: &u16, ngb: u16) {
        let mut stream = TcpStream::connect(("127.0.0.1", server.to_owned())).unwrap();

        let message = Message::new(
            MessageID::new(),
            Type::Chunk(chunk.to_vec()),
            Some(NodeID::from_u16(ngb)),
        );

        let mut buffer = bincode::serialize(&message).unwrap();
        stream.write_all(&buffer);
        // add_to_local_memory(&message, servers[index]);
        stream.flush();
    }

    pub fn add_neighbour(chunk: &[Message], index: usize, servers: Vec<u16>) -> &[Message] {
        for message in chunk.to_owned().iter_mut() {
            if index == servers.len() - 1 {
                message.neighbour = Some(NodeID::from_u16(servers[0]));
            } else {
                message.neighbour = Some(NodeID::from_u16(servers[index + 1]));
            }
        }
        chunk
    }
    // pub fn dispatch_messages(messages: Vec<Message>, servers: Vec<u16>) {

    //     let chunked_messages = messages.chunks(messages.len() / servers.len());
    //     println!("Create {} chunks", chunked_messages.len());

    //     chunked_messages
    //         .into_iter()
    //         .enumerate()
    //         .for_each(|(index, chunk)| {
    //             let mut stream = TcpStream::connect(("127.0.0.1", servers[index])).unwrap();
    //             println!("Chunk {}", index);
    //             for message in chunk.to_owned().iter_mut() {
    //                 if index == servers.len() - 1 {
    //                     message.neighbour = Some(NodeID::from_u16(servers[0]));
    //                 } else {
    //                     message.neighbour = Some(NodeID::from_u16(servers[index + 1]));
    //                 }
    //                 println!("Message: {:#?}", &message);
    //                 let mut buffer = bincode::serialize(&message).unwrap();
    //                 stream.write_all(&buffer);
    //                 add_to_local_memory(&message, servers[index]);
    //                 stream.flush();
    //             }
    //         });
    // }

    fn add_to_local_memory(message: &Message, server: u16) {
        println!("{}", server);
        // println!("Added {:#?} to local memory", message)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{self, Read, Write},
        net::TcpListener,
        sync::{Arc, Mutex},
        thread, vec,
    };

    use crate::message_dispatch::Client;

    // #[test]
    // fn dispatch_3_messages_3_servers() {
    //     let servers: Vec<u16> = vec![8004, 8005, 8006];
    //     for port in servers.clone() {
    //         let servers: Vec<u16> = vec![8004, 8005, 8006];
    //         let handle = thread::spawn(move || {
    //             let messages = Client::create_message_stack(3);
    //             let listener = TcpListener::bind(("127.0.0.1", port.to_owned())).unwrap();

    //             let mut buffer: Vec<u8> = vec![];

    //             for stream in listener.incoming() {
    //                 let Ok(mut stream) = stream else {
    //                     return Err(io::Error::new(
    //                         io::ErrorKind::NotFound,
    //                         "Could not unwrap TcpStream",
    //                     ));
    //                 };
    //                 let mut buffer = [0; 1024];
    //                 Client::dispatch_messages(messages.clone(), servers.clone());
    //                 assert_eq!(stream.read(&mut buffer).unwrap(), 1);
    //                 assert_eq!(messages.len(), 3);
    //             }
    //             Ok(())
    //         });
    //     }
    // }
}
