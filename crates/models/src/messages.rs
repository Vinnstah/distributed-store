use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    id: MessageID,
    body: Transaction
}

impl Message {
    pub fn new(id: MessageID, body: Transaction) -> Self {
        Self { id, body }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Transaction {
    Init(Init),
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