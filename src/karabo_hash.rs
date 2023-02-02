use std::fmt;
use std::ops::{Deref, Index};
use wasm_bindgen::prelude::wasm_bindgen;

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
}

impl Attributes {
    pub fn new() -> Self {
        Attributes { store: Vec::new() }
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
        for attr in self.store.iter() {
            if attr.key == key {
                return Some(&attr.value);
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut HashValue> {
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
        self.insert_index(key, value);
    }

    pub(crate) fn insert_index(&mut self, key: &str, value: HashValue) {
        self.store.push(Attribute::new(key.to_string(), value));
    }
}

impl PartialEq for Attributes {
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
        }
        true
    }
}

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
}

impl Hash {
    pub fn new() -> Self {
        Hash { store: Vec::new() }
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

    pub fn get_index(&self, index: usize) -> Option<&Node> {
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
        for node in self.store.iter() {
            if node.key == key {
                return Some(&node.value);
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut HashValue> {
        if self.store.is_empty() {
            return None;
        }

        for node in self.store.iter_mut() {
            if node.key == key {
                return Some(&mut node.value);
            }
        }
        None
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
        self.store.push(Node::new(key.to_string(), value, attrs));
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

#[derive(Clone, Debug, PartialEq)]
pub struct Schema {
    pub class_id: String,
    pub hash: Hash,
}

impl Schema {
    pub fn new(class_id: String, hash: Hash) -> Schema {
        Schema { class_id, hash }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HashValue {
    Bool(bool),
    VectorBool(Vec<bool>),
    Char(char),
    VectorChar(Vec<char>),
    UInt8(u8),
    VectorUInt8(Vec<u8>),
    Int8(i8),
    VectorInt8(Vec<i8>),
    UInt16(u16),
    VectorUInt16(Vec<u16>),
    Int16(i16),
    VectorInt16(Vec<i16>),
    UInt32(u32),
    VectorUInt32(Vec<u32>),
    Int32(i32),
    VectorInt32(Vec<i32>),
    UInt64(u64),
    VectorUInt64(Vec<u64>),
    Int64(i64),
    VectorInt64(Vec<i64>),
    Float32(f32),
    VectorFloat32(Vec<f32>),
    Float64(f64),
    VectorFloat64(Vec<f64>),
    String(String),
    VectorString(Vec<String>),
    Hash(Hash),
    VectorHash(Vec<Hash>),
    Schema(Schema),
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

impl fmt::Display for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HashValue::Bool(x) => write!(f, "BOOL {x}"),
            HashValue::VectorBool(x) => write!(f, "VECTOR_BOOL {x:?}"),
            HashValue::Char(x) => write!(f, "CHAR {x}"),
            HashValue::VectorChar(x) => write!(f, "VECTOR_CHAR {x:?}"),
            HashValue::UInt8(x) => write!(f, "UINT8 {x}"),
            HashValue::VectorUInt8(x) => write!(f, "VECTOR_UINT8 {x:?}"),
            HashValue::Int8(x) => write!(f, "INT8 {x}"),
            HashValue::VectorInt8(x) => write!(f, "VECTOR_INT8 {x:?}"),
            HashValue::UInt16(x) => write!(f, "UINT16 {x}"),
            HashValue::VectorUInt16(x) => write!(f, "VECTOR_UINT16 {x:?}"),
            HashValue::Int16(x) => write!(f, "INT16 {x}"),
            HashValue::VectorInt16(x) => write!(f, "VECTOR_INT16 {x:?}"),
            HashValue::UInt32(x) => write!(f, "UINT32 {x}"),
            HashValue::VectorUInt32(x) => write!(f, "VECTOR_UINT32 {x:?}"),
            HashValue::Int32(x) => write!(f, "INT32 {x}"),
            HashValue::VectorInt32(x) => write!(f, "VECTOR_INT32 {x:?}"),
            HashValue::UInt64(x) => write!(f, "UINT64 {x}"),
            HashValue::VectorUInt64(x) => write!(f, "VECTOR_UINT64 {x:?}"),
            HashValue::Int64(x) => write!(f, "INT64 {x}"),
            HashValue::VectorInt64(x) => write!(f, "VECTOR_INT64 {x:?}"),
            HashValue::Float32(x) => write!(f, "FLOAT {x}"),
            HashValue::VectorFloat32(x) => write!(f, "VECTOR_FLOAT {x:?}"),
            HashValue::Float64(x) => write!(f, "DOUBLE {x}"),
            HashValue::VectorFloat64(x) => write!(f, "VECTOR_DOUBLE {x:?}"),
            HashValue::String(x) => write!(f, "STRING {x}"),
            HashValue::VectorString(x) => write!(f, "VECTOR_STRING {x:?}"),
            _ => write!(f, "undefined"),
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

pub fn get_hashtype(value: &HashValue) -> u32 {
    match value {
        HashValue::Bool(_) => 0,
        HashValue::VectorBool(_) => 1,
        HashValue::Char(_) => 2,
        HashValue::VectorChar(_) => 3,
        HashValue::Int8(_) => 4,
        HashValue::VectorInt8(_) => 5,
        HashValue::UInt8(_) => 6,
        HashValue::VectorUInt8(_) => 7,
        HashValue::Int16(_) => 8,
        HashValue::VectorInt16(_) => 9,
        HashValue::UInt16(_) => 10,
        HashValue::VectorUInt16(_) => 11,
        HashValue::Int32(_) => 12,
        HashValue::VectorInt32(_) => 13,
        HashValue::UInt32(_) => 14,
        HashValue::VectorUInt32(_) => 15,
        HashValue::Int64(_) => 16,
        HashValue::VectorInt64(_) => 17,
        HashValue::UInt64(_) => 18,
        HashValue::VectorUInt64(_) => 19,
        HashValue::Float32(_) => 20,
        HashValue::VectorFloat32(_) => 21,
        HashValue::Float64(_) => 22,
        HashValue::VectorFloat64(_) => 23,
        HashValue::String(_) => 28,
        HashValue::VectorString(_) => 29,
        HashValue::Hash(_) => 30,
        HashValue::VectorHash(_) => 31,
        HashValue::Schema(_) => 32,
    }
}
