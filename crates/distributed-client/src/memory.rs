use std::{collections::HashMap, hash::Hash};

use models::{message::CircularList, node::NodeID};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientMemory<T>
where
    T: Eq + PartialEq + Hash,
{
    pub list_of_servers: CircularList<NodeID>,
    pub value_node_map: HashMap<T, Vec<NodeID>>,
}

impl<T> ClientMemory<T>
where
    T: Eq + PartialEq + Hash,
{
    pub fn new(list_of_servers: CircularList<NodeID>) -> Self {
        Self {
            list_of_servers,
            value_node_map: HashMap::new(),
        }
    }

    pub fn add_new_server(&mut self, id: NodeID) {
        self.list_of_servers.elements.push(id)
    }

    pub fn insert_value_for_nodes(&mut self, key: T, nodes: Vec<NodeID>) {
        println!("Added value for nodes: {:#?}", nodes);
        self.value_node_map.insert(key, nodes);
    }

    pub fn delete_value(&mut self, key: T) {
        self.value_node_map.remove(&key);
    }
}
