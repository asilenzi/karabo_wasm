use std::fmt;

use std::collections::HashMap;

use crate::types::{get_hashtype, HashValue};

#[derive(Clone, Debug)]
pub struct Attribute {
    pub key: String,
    pub value: HashValue,
}

impl Attribute {
    #[inline]
    fn new(key: String, value: HashValue) -> Attribute {
        Attribute { key, value }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Attributes {
    store: Vec<Attribute>,
    key_map: HashMap<String, usize>,
}

impl Attributes {
    pub fn new() -> Self {
        Attributes {
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

    pub fn get_index(&self, index: usize) -> Option<&Attribute> {
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
        let position = self.key_map.get(key);
        match position {
            Some(idx) => Some(&self.store[*idx].value),
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

    #[inline]
    pub fn insert(&mut self, key: &str, value: HashValue) {
        self.insert_index(key, value);
    }

    pub(crate) fn insert_index(&mut self, key: &str, value: HashValue) {
        let position = self.key_map.get(key);
        let new_attr = Attribute::new(key.to_string(), value);
        match position {
            Some(idx) => {
                self.store[*idx] = new_attr;
            }
            None => {
                let key = key.to_string();
                self.key_map.insert(key, self.store.len());
                self.store.push(new_attr);
            }
        }
    }
}

pub struct AttributesIterator<'a> {
    attrs: &'a Attributes,
    index: usize,
}

impl<'a> Iterator for AttributesIterator<'a> {
    type Item = Attribute;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.attrs.get_index(self.index);
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

impl PartialEq for Attributes {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        if self
            .keys()
            .iter()
            .filter(|&a| {
                other.get(a).is_some()
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
        }
        true
    }
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ret: Vec<String> = self
            .keys()
            .iter()
            .map(|key| format!("'{}' => {}\n", key, &self.get(key).unwrap()))
            .collect();
        let ret = ret.join(",");
        write!(f, "{{{}}}", ret)
    }
}
