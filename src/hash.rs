use crate::attributes::Attributes;
use crate::types::{get_hashtype, HashValue};
use std::collections::HashMap;
use std::fmt;
use std::ops::{Deref, Index};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug)]
pub struct Node {
    pub key: String,
    pub value: HashValue,
    pub attrs: Attributes,
}

impl Node {
    fn new(key: String, value: HashValue, attrs: Attributes) -> Node {
        Node { key, value, attrs }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Hash {
    store: Vec<Node>,
    key_map: HashMap<String, usize>,
}

impl Hash {
    pub fn new() -> Self {
        Hash {
            store: Vec::new(),
            key_map: HashMap::new(),
        }
    }

    pub fn keys(&self) -> Vec<String> {
        if self.store.is_empty() {
            return Vec::new();
        }
        return self
            .store
            .iter()
            .map(|x| x.key.clone())
            .collect::<Vec<String>>();
    }

    pub(crate) fn get_index(&self, index: usize) -> Option<&Node> {
        self.store.get(index)
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&HashValue> {
        if self.store.is_empty() {
            return None;
        }
        let parts: Vec<&str> = key.split(".").collect();
        let position = self.key_map.get(parts[0]);
        match position {
            Some(idx) => match &self.store[*idx].value {
                HashValue::Hash(x) if parts.len() > 1 => {
                    let key = parts[1..].join(".");
                    x.get(&key)
                }
                _x => Some(_x),
            },
            None => None,
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut HashValue> {
        if self.store.is_empty() {
            return None;
        }
        let position = self.key_map.get(key);
        match position {
            Some(idx) => Some(&mut self.store[*idx].value),
            None => None,
        }
    }

    pub fn get_attributes(&self, key: &str) -> Option<&Attributes> {
        if self.store.is_empty() {
            return None;
        }
        for node in self.store.iter() {
            if node.key == key {
                return Some(&node.attrs);
            }
        }
        None
    }

    pub fn get_mut_attributes(&mut self, key: &str) -> Option<&mut HashValue> {
        if self.store.is_empty() {
            return None;
        }

        for attr in self.store.iter_mut() {
            if attr.key == key {
                return Some(&mut attr.value);
            }
        }
        None
    }

    #[inline]
    pub fn insert(&mut self, key: &str, value: HashValue) {
        let attrs = Attributes::new();
        self.insert_index_attrs(key, value, attrs);
    }

    #[inline]
    pub fn insert_attrs(&mut self, key: &str, value: HashValue, attrs: Attributes) {
        self.insert_index_attrs(key, value, attrs);
    }

    pub(crate) fn insert_index_attrs(&mut self, key: &str, value: HashValue, attrs: Attributes) {
        let position = self.key_map.get(key);
        let new_node = Node::new(key.to_string(), value, attrs);
        match position {
            Some(idx) => {
                self.store[*idx] = new_node;
            }
            None => {
                let key = key.to_string();
                self.key_map.insert(key, self.store.len());
                self.store.push(new_node);
            }
        }
    }
}

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let other_keys = other.keys();
        if self
            .keys()
            .iter()
            .zip(&other_keys)
            .filter(|&(a, b)| {
                a != b
                    && get_hashtype(self.get(a).expect("")) != get_hashtype(other.get(a).expect(""))
            })
            .count()
            != 0
        {
            return false;
        }
        for key in self.keys() {
            if self.get(&key) != other.get(&key) {
                return false;
            }
            if self.get_attributes(&key) != other.get_attributes(&key) {
                return false;
            }
        }
        true
    }
}

pub struct HashIterator<'a> {
    hash: &'a Hash,
    index: usize,
}

impl<'a> Iterator for HashIterator<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.hash.get_index(self.index);
        self.index += 1;
        match result {
            Some(x) => {
                self.index += 1;
                Some(x.clone())
            }
            None => None,
        }
    }
}

impl<'a> Index<&'a str> for Hash {
    type Output = HashValue;

    fn index(&self, index: &str) -> &HashValue {
        match self.get(index) {
            Some(value) => value,
            None => panic!("Missing Key {}", index),
        }
    }
}

impl Index<String> for Hash {
    type Output = HashValue;

    fn index(&self, index: String) -> &HashValue {
        self.index(index.deref())
    }
}

impl<'a> Index<&'a String> for Hash {
    type Output = HashValue;

    fn index(&self, index: &String) -> &HashValue {
        self.index(index.deref())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        for key in self.keys() {
            let value = &self[&key];
            match self.get_attributes(&key) {
                None => {
                    result = write!(f, "{} {:?}", key, value);
                }
                Some(attrs) => {
                    result = write!(f, "{} {:?} {:?}", key, value, attrs);
                }
            }
            if result.is_err() {
                break;
            }
        }
        result
    }
}
