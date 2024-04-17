use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    id: MessageID,
    body: Body
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Body {
    Init(Init),
    Gossip(Gossip),
    Delete(Delete),
    Insert(Insert)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Init {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Gossip {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Delete {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Insert {}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
struct MessageID(String);