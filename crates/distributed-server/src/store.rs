use crate::node::NodeID;
use models::messages::Transaction;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Store {
    elements: Elements,
    replicated_at_neighbour: NodeID,
    transaction_queue: VecDeque<Transaction>,
}

impl Store {
    pub fn new(replicated_at_neighbour: NodeID) -> Self {
        Self {
            elements: Elements(HashMap::new()),
            replicated_at_neighbour,
            transaction_queue: VecDeque::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Elements(HashMap<String, String>);

pub trait Transactions {
    type Key;
    type Value;

    fn insert(&mut self, element: Element<Self::Key, Self::Value>);
    fn get(&self, key: String) -> Option<Element<Self::Key, Self::Value>>;
    fn delete(&mut self, key: String) -> bool;
}

impl Transactions for Store {
    type Key = String;
    type Value = String;

    fn insert(&mut self, element: Element<String, String>) {
        self.elements.0.entry(element.key).or_insert(element.value);
    }

    fn get(&self, key: String) -> Option<Element<String, String>> {
        match self.elements.0.get(&key) {
            Some(value) => Some(Element::new(key, value.into())),
            None => None,
        }
    }

    fn delete(&mut self, key: String) -> bool {
        match self.elements.0.remove_entry(&key) {
            Some(_) => true,
            None => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Element<K, V> {
    key: K,
    value: V,
}

impl<K, V> Element<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        node::NodeID,
        store::{Element, Elements, Store, Transactions},
    };

    #[test]
    fn new_store() {
        let store = Store::new(NodeID::new());
        assert_eq!(store.elements, Elements(HashMap::new()));
    }

    #[test]
    fn equality() {
        assert_eq!(Store::new(NodeID::new()), Store::new(NodeID::new()));
    }

    #[test]
    fn inequality() {
        assert_ne!(
            Store::new(NodeID::from(&"1")),
            Store::new(NodeID::from(&"2"))
        );
    }

    #[test]
    fn insert_store_len() {
        let mut store = Store::new(NodeID::new());
        assert_eq!(store.elements.0.len(), 0);
        store.insert(Element::new("Link".to_string(), "Zelda".to_string()));
        assert_eq!(store.elements.0.len(), 1);
    }

    #[test]
    fn insert_store_value() {
        let mut store = Store::new(NodeID::new());
        store.insert(Element::new("Link".to_string(), "Zelda".to_string()));
        assert_eq!(
            store.elements.0.get("Link"),
            Some("Zelda".to_string()).as_ref()
        );
    }

    #[test]
    fn get_store_value() {
        let mut store = Store::new(NodeID::new());
        store.insert(Element::new("Link".to_string(), "Zelda".to_string()));
        assert_eq!(
            store.get("Link".to_string()),
            Some(Element::new("Link".to_string(), "Zelda".to_string()))
        );
    }
}
