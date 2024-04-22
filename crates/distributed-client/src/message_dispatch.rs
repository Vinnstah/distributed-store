use std::{
    borrow::BorrowMut,
    io::Write,
    net::TcpStream,
    process::Child,
    sync::{Arc, Mutex},
    thread,
};

use models::{
    message::{self, Insert, Message, MessageID, Transaction, Type},
    node::NodeID,
};
use rand::random;

pub fn create_message_stack(amount: usize) -> Vec<Message> {
    let mut stack_of_messages: Vec<Message> = vec![];
    for _ in 0..amount {
        stack_of_messages.push({
            Message::new(
                MessageID::new(),
                Type::Request(Transaction::Insert(Insert::new(
                    MessageID::new(),
                    random::<u16>(),
                ))),
                None,
            )
        });
    }
    println!("Create stack of {} messages", amount);
    stack_of_messages
}

pub fn dispatch_messages(messages: Vec<Message>, servers: Vec<u16>) {

    let chunked_messages = messages.chunks(messages.len() / servers.len());
    println!("Create {} chunks", chunked_messages.len());

    chunked_messages
        .into_iter()
        .enumerate()
        .for_each(|(index, chunk)| {
            let mut stream = TcpStream::connect(("127.0.0.1", servers[index])).unwrap();
            println!("Chunk {}", index);
            for message in chunk.to_owned().iter_mut() {
                if index == servers.len() - 1 {
                    message.neighbour = Some(NodeID::from_u16(servers[0]));
                } else {
                    message.neighbour = Some(NodeID::from_u16(servers[index + 1]));
                }
            }
                let mut buffer = bincode::serialize(&chunk.to_vec()).unwrap();
                stream.write_all(&buffer);
                // add_to_local_memory(&message, servers[index]);
                // stream.flush();
        });
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

#[cfg(test)]
mod tests {
    use std::{
        io::{self, Read, Write},
        net::TcpListener,
        sync::{Arc, Mutex},
        thread, vec,
    };

    use crate::message_dispatch::create_message_stack;

    use super::dispatch_messages;

    #[test]
    fn dispatch_3_messages_3_servers() {
        let servers: Vec<u16> = vec![8004, 8005, 8006];
        for port in servers.clone() {
            let servers: Vec<u16> = vec![8004, 8005, 8006];
            let handle = thread::spawn(move || {
                let messages = create_message_stack(3);
                let listener = TcpListener::bind(("127.0.0.1", port.to_owned())).unwrap();

                let mut buffer: Vec<u8> = vec![];

                for stream in listener.incoming() {
                    let Ok(mut stream) = stream else {
                        return Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            "Could not unwrap TcpStream",
                        ));
                    };
                    let mut buffer = [0; 1024];
                    dispatch_messages(messages.clone(), servers.clone());
                    assert_eq!(stream.read(&mut buffer).unwrap(), 1);
                    assert_eq!(messages.len(), 3);
                }
                Ok(())
            });
        }
    }
}
