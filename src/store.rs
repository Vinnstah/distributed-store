use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};

use crate::node::NodeID;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Store {
    elements: Vec<Element>,
    transaction_queue: VecDeque<HashMap<String, String>>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Element {
    data: HashMap<String, String>,
    replicated_at_neighbour: NodeID
}
