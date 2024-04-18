use crate::store::Store;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Node {
    id: NodeID,
    #[serde(rename = "ngb")]
    neighbour: NodeID,
    store: Store,
}

impl Node {
    pub fn new(id: NodeID, neighbour: NodeID) -> Self {
        Self {
            id,
            neighbour: neighbour.clone(),
            store: Store::new(neighbour),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct NodeID(String);

impl NodeID {
    pub fn new() -> Self {
        NodeID(String::from("id"))
    }

    pub fn from<T: AsRef<str>>(id: T) -> Self {
        NodeID(id.as_ref().to_string())
    }

    pub fn from_u16(id: u16) -> Self {
        NodeID(id.to_string())
    }
}