use serde::{Deserialize, Serialize};
use crate::store::Store;


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Node {
    id: NodeID,
    #[serde(rename = "ngb")]
    neighbours: Vec<Node>,
    store: Store
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct NodeID(String);