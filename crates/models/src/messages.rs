use serde::{Deserialize, Serialize};

use crate::node::Node;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: MessageID,
    #[serde(rename = "type")]
    pub message_type: Type
}

impl Message {
    pub fn new(id: MessageID, message_type: Type) -> Self {
        Self { id, message_type }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Request(Transaction),
    Response(Response)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Response {
    InitOk(Node)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Transaction {
    Init,
    Gossip(Gossip),
    Delete(Delete),
    Insert(Insert)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Init {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Gossip {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Delete {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Insert {}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageID(String);

impl MessageID {
    pub fn new() -> Self {
        MessageID(String::from("id"))
    }

    pub fn from(id: &dyn AsRef<str>) -> Self {
        MessageID(id.as_ref().to_string())
    }
}