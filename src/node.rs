use serde::{Deserialize, Serialize};
use crate::store::Store;


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Node {
    id: NodeID,
    #[serde(rename = "ngb")]
    neighbour: NodeID,
    store: Store
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct NodeID(String);