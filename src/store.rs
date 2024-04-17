use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

use crate::{messages::Transaction, node::NodeID};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Store {
    elements: Elements,
    replicated_at_neighbour: NodeID,
    transaction_queue: VecDeque<Transaction>,
}

impl Store {
    pub fn new(
        replicated_at_neighbour: NodeID,
    ) -> Self {
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
    fn insert(&mut self, element: Element);
    fn get(&self, key: String) -> Option<Element>;
    fn delete(&mut self, key: String) -> bool;
}

impl Transactions for Store {
    fn insert(&mut self, element: Element) {
        self.elements
            .0
            .entry(element.key)
            .or_insert(element.value);
    }

    fn get(&self, key: String) -> Option<Element> {
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
pub struct Element {
    key: String,
    value: String,
}

impl Element {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
