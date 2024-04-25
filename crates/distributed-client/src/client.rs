use models::{
    message::{CircularList, Insert, Message, MessageID, Transaction},
    node::NodeID,
    tcp_client::Stream,
};
use rand::random;

use crate::memory::ClientMemory;

pub struct Client {
    pub stream: Stream,
    pub memory: ClientMemory<String>,
    pub servers: Vec<NodeID>,
    pub message_stack: Vec<Transaction>
}

impl Client {
    pub fn new(stream: Stream, servers: Vec<NodeID>) -> Self {
        Self {
            stream,
            memory: ClientMemory::new(CircularList::new(servers)),
            servers: vec![],
            message_stack: vec![]
        }
    }

    pub fn create_message_stack(&mut self, amount: usize)  {
        for _ in 0..amount {
            self.message_stack.push(Transaction::Insert(Insert::new(
                MessageID::new(),
                random::<u16>(),
            )));
        }
        println!("Created stack of {:#?} messages", self.message_stack);
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

    pub fn add_to_local_memory(&mut self, message: &Message, server: &NodeID) {
        match &message.message_type {
            models::message::Type::Request(_) => todo!(),
            models::message::Type::Chunk(chunk) => {
                chunk.iter().for_each(|t| {
                    self.handle_transaction(
                        t,
                        vec![server.clone(), message.clone().neighbour.unwrap()],
                    )
                });
            }
            models::message::Type::Response(_) => todo!(),
        }
    }

    fn handle_transaction(&mut self, transaction: &Transaction, nodes: Vec<NodeID>) {
        match transaction {
            Transaction::Init => todo!(),
            Transaction::Gossip(_) => todo!(),
            Transaction::Delete(_) => todo!(),
            Transaction::Insert(payload) => self
                .memory
                .insert_value_for_nodes(payload.value.to_string(), nodes),
            Transaction::Fetch(_) => return,
        }
    }
}
