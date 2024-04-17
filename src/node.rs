use crate::store::Store;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
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
