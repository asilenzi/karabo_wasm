use std::fmt;

pub struct Attribute {
    pub key: String,
    pub value: HashValue,
}

impl Attribute {
    #[inline]
    fn new(key: String, value: HashValue) -> Attribute {
        Attribute {
            key: key,
            value: value }
    }
}

pub struct Attributes {
    store: Vec<Attribute>
}

impl Attributes {
    pub fn new() -> Self {
        Attributes {
            store: Vec::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<&HashValue> {
        if self.store.len() == 0 {
            return None;
        }
        for attr in self.store.iter() {
            if attr.key == key {
                return Some(&attr.value);
            }
        }
        return None;
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut HashValue> {
        if self.store.len() == 0 {
            return None;
        }
    
        for attr in self.store.iter_mut() {
            if attr.key == key {
                return Some(&mut attr.value);
            }
        }
        return None;
    }

    #[inline]
    pub fn insert(&mut self, key: &str, value: HashValue) {
        self.insert_index(key, value);
    }

    pub(crate) fn insert_index(&mut self, key: &str, value: HashValue) {
        self.store.push(Attribute::new(key.to_string(), value));
    }

}


pub struct Node {
    pub key: String,
    pub value: HashValue,
    pub attrs: Attributes
}

impl Node {
    fn new(key: String, value: HashValue, attrs: Attributes) -> Node {
        Node {
            key: key,
            value: value,
            attrs: attrs,
        }
    }
}

pub struct Hash {
    store: Vec<Node>
}

impl Hash {
    pub fn new() -> Self {
        Hash {
            store: Vec::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<&HashValue> {
            if self.store.len() == 0 {
            return None;
        }
        for node in self.store.iter() {
            if node.key == key {
                return Some(&node.value);
            }
        }
        return None;
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut HashValue> {
        if self.store.len() == 0 {
            return None;
        }
    
        for node in self.store.iter_mut() {
            if node.key == key {
                return Some(&mut node.value);
            }
        }
        return None;
    }

    pub fn get_attributes(&self, key: &str) -> Option<&Attributes> {
        if self.store.len() == 0 {
            return None;
        }
        for node in self.store.iter() {
            if node.key == key {
                return Some(&node.attrs);
            }
        }
        return None;
    }

    pub fn get_mut_attributes(&mut self, key: &str) -> Option<&mut HashValue> {
        if self.store.len() == 0 {
            return None;
        }

        for attr in self.store.iter_mut() {
            if attr.key == key {
                return Some(&mut attr.value);
            }
        }
        return None;
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

pub struct Schema {
    pub class_id: String,
    pub hash: Hash,
}

impl Schema {
    pub fn new(class_id: String, hash: Hash) -> Schema {
        Schema {
            class_id: class_id,
            hash: hash,
        }
    }
}

pub enum HashValue {
    Bool(bool),
    Char(u8),  // a 8 bit char
    VectorChar(Vec<u8>),  // a 8 bit char
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

impl fmt::Display for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HashValue::Bool(x) => write!(f, "BOOL {x}"),
            HashValue::Char(x) => write!(f, "CHAR {}", *x as char),
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
