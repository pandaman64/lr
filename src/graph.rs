use grammer::Character;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Arena<T> {
    pub nodes: Vec<Node<T>>,
    pub edges: Vec<HashMap<Character, usize>>,
}

#[derive(Debug)]
pub struct Node<T> {
    pub id: usize,
    pub value: T,
}

impl<T: Eq> Arena<T> {
    pub fn new() -> Self {
        Arena {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn get(&self, v: &T) -> Option<&Node<T>> {
        for n in self.nodes.iter() {
            if n.value == *v {
                return Some(n);
            }
        }
        None
    }

    pub fn get_mut(&mut self, v: &T) -> Option<&mut Node<T>> {
        for n in self.nodes.iter_mut() {
            if n.value == *v {
                return Some(n);
            }
        }
        None
    }

    pub fn push(&mut self, v: T) {
        self.edges.push(HashMap::new());
        let l = self.nodes.len();
        self.nodes.push(Node {
            id: l,
            value: v,
        });
    }
}

