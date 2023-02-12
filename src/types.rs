use std::fmt;

use crate::hash::Hash;
use crate::schema::Schema;

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

impl HashValue {
    pub fn as_i8(&self) -> Option<i8> {
        match *self {
            HashValue::Int8(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_u8(&self) -> Option<u8> {
        match *self {
            HashValue::UInt8(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_i16(&self) -> Option<i16> {
        match *self {
            HashValue::Int16(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_u16(&self) -> Option<u16> {
        match *self {
            HashValue::UInt16(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            HashValue::Int32(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_u32(&self) -> Option<u32> {
        match *self {
            HashValue::UInt32(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            HashValue::Int64(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            HashValue::UInt64(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_f32(&self) -> Option<f32> {
        match *self {
            HashValue::Float32(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            HashValue::Float64(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_hash(&self) -> Option<&Hash> {
        match self {
            HashValue::Hash(value) => Some(value),
            _ => None,
        }
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
