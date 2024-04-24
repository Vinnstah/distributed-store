use std::{collections::VecDeque, slice::Iter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::node::{self, Node, NodeID};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: MessageID,
    #[serde(rename = "type")]
    pub message_type: Type,
    #[serde(rename = "ngb")]
    pub neighbour: Option<NodeID>,
}

impl Message {
    pub fn new(id: MessageID, message_type: Type, neighbour: Option<NodeID>) -> Self {
        Self {
            id,
            message_type,
            neighbour,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Request(Transaction),
    Chunk(Vec<Transaction>),
    Response(Response),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Response {
    InitOk(Node),
    InsertOk(Node)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Transaction {
    Init,
    Gossip(Gossip),
    Delete(Delete),
    Insert(Insert),
    Fetch(Fetch),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Init {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Gossip {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Delete {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Insert {
    id: MessageID,
    value: u16,
}

impl Insert {
    pub fn new(id: MessageID, value: u16) -> Self {
        Self { id, value }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Fetch {}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageID(String);

impl MessageID {
    pub fn new() -> Self {
        MessageID(Uuid::new_v4().to_string())
    }

    pub fn from(id: &dyn AsRef<str>) -> Self {
        MessageID(id.as_ref().to_string())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct CircularList<T> {
    pub elements: Vec<T>,
}

impl<NodeID> CircularList<NodeID> {
    pub fn new(elements: Vec<NodeID>) -> Self {
        Self {
            elements: Vec::from(elements),
        }
    }

    pub fn neighbour(&self, index: usize) -> &NodeID {
        if index == self.elements.len() - 1 {
            return self.elements.get(0).expect("No neighbour at 0");
        } else {
            return &self.elements[index + 1];
        }
    }
}

pub trait CircularIterator {
    type Type;
    fn next_nonoptional(&mut self) -> Self::Type;
}

impl Iterator for CircularList<NodeID> {
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        if self.elements.iter().next() == None {
            println!("Herd2");
            return Some(self.elements[0].clone())
        } else {
            println!("Her12");
            Some(self.elements.iter().next().expect("Failed to find next element").clone())
        }
    }
}

impl CircularIterator for CircularList<u16> {
    type Type = u16;

    fn next_nonoptional(&mut self) -> Self::Type {
        println!("{:#?}", self.elements.iter_mut().next());
        if self.elements.iter_mut().next() == None {
            println!("Herd2");
            return self.elements[0].clone()
        } else {
            println!("Her12");
            self.elements.iter_mut().next().expect("Failed to find next element").clone()
        }
    }
    
}

impl CircularIterator for Vec<u16> {
    type Type = u16;

    fn next_nonoptional(&mut self) -> Self::Type {
        println!("{:#?}", self.iter().next());
        if self.iter().next() == None {
            return self[0]
        } else {
            self.iter().next().expect("Failed to find next element").clone()
        }
    }
}
