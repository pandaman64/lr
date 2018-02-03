use grammer::Character;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Arena<T> {
    pub nodes: Vec<Node<T>>,
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub children: HashMap<Character, *mut Node<T>>,
}

impl<T: Eq> Arena<T> {
    pub fn new() -> Self {
        Arena {
            nodes: vec![]
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
        self.nodes.push(Node {
            value: v,
            children: HashMap::new(),
        });
    }
}

