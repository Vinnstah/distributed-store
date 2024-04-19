use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::node::{Node, NodeID};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: MessageID,
    #[serde(rename = "type")]
    pub message_type: Type,
    #[serde(rename = "ngb")]
    pub neighbour: NodeID,
}

impl Message {
    pub fn new(id: MessageID, message_type: Type, neighbour: NodeID) -> Self {
        Self { id, message_type, neighbour }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Request(Transaction),
    Response(Response),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Response {
    InitOk(Node),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Transaction {
    Init,
    Gossip(Gossip),
    Delete(Delete),
    Insert(Insert),
    Fetch(Fetch)
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
    value: u16
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
    pub elements: VecDeque<T>,
}

impl<NodeID> CircularList<NodeID> {
    pub fn new(elements: Vec<NodeID>) -> Self {
        Self { elements: VecDeque::from(elements) }
    }
    
    pub fn neighbour(&self, index: usize) -> &NodeID {
        if index == self.elements.len() - 1 {
            return self.elements.get(0).expect("No neighbour at 0")
        } else {
            return &self.elements[index+1]
        }
    }
}
