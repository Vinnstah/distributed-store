use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    id: MessageID,
    body: Transaction
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
struct MessageID(String);