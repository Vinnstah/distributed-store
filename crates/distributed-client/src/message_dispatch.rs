use std::net::TcpStream;

use models::{
    message::{Message, MessageID, Transaction, Type},
    node::NodeID,
    tcp_client::TcpClient,
};

use crate::client::Client;

impl Client {
    pub fn dispatch_messages(&mut self) {
        let message_stack = self.message_stack.clone();
        let chunked_messages = message_stack.chunks(10);
        println!("Create {} chunks", chunked_messages.len());

        chunked_messages
            .into_iter()
            .enumerate()
            .for_each(|(index, chunk)| {
                let server_index = (index + 1) % self.servers.len();
                self.dispatch_chunk(
                    chunk,
                    self.servers[server_index].clone(),
                    self.servers[(index + 2) % self.servers.len()].clone(),
                );
            });
    }

    pub fn dispatch_chunk(&mut self, chunk: &[Transaction], server: NodeID, ngb: NodeID) {
        let mut stream = TcpStream::connect((
            "127.0.0.1",
            Into::<u16>::into(<NodeID as Clone>::clone(&server)),
        ))
        .unwrap();

        let message = Message::new(MessageID::new(), Type::Chunk(chunk.to_vec()), Some(ngb));

        let mut buffer = bincode::serialize(&message).unwrap();

        self.stream
            .write(buffer, Into::<u16>::into(<NodeID as Clone>::clone(&server)))
            .unwrap();

        self.add_to_local_memory(&message, &server)
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
