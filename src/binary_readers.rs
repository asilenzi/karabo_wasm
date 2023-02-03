use std::io::{Read, Result};

use crate::karabo_hash::{Attributes, Hash, HashValue, Schema};

fn read_vbool<R: Read>(buf: &mut R) -> Result<Vec<bool>> {
    let size_ = read_u32(buf).expect("error reading key size");
    let size = usize::try_from(size_).unwrap();
    let mut arr = vec![0u8; size];
    buf.read_exact(&mut arr).expect("error reading value");
    let ret = arr.iter().map(|&e| e != 0_u8).collect();
    Ok(ret)
}

fn read_vchar<R: Read>(buf: &mut R) -> Result<Vec<char>> {
    let size_ = read_u32(buf).expect("error reading key size");
    let size = usize::try_from(size_).unwrap();
    let mut arr = vec![0u8; size];
    buf.read_exact(&mut arr).expect("error reading value");
    let ret = arr.iter().map(|&e| e as char).collect();
    Ok(ret)
}

fn read_vu8<R: Read>(buf: &mut R) -> Result<Vec<u8>> {
    let size_ = read_u32(buf).expect("error reading key size");
    let size = usize::try_from(size_).unwrap();
    let mut arr = vec![0u8; size];
    buf.read_exact(&mut arr).expect("error reading value");
    Ok(arr)
}

fn read_vi8<R: Read>(buf: &mut R) -> Result<Vec<i8>> {
    let size_ = read_u32(buf).expect("error reading key size");
    let size = usize::try_from(size_).unwrap();
    let mut arr = vec![0u8; size];
    buf.read_exact(&mut arr).expect("error reading value");
    let ret = arr.iter().map(|&e| e as i8).collect();
    Ok(ret)
}

fn read_vi16<R: Read>(buf: &mut R) -> Result<Vec<i16>> {
    let size_ = read_u32(buf).expect("error reading key size");
    let size = usize::try_from(size_).unwrap();
    let chunk_size: usize = 2;
    let mut arr = vec![0u8; size * chunk_size];
    buf.read_exact(&mut arr).expect("error reading value");
    let ret = arr
        .chunks(chunk_size)
        .map(|chunk| {
            let sized: [u8; 2] = chunk.try_into().expect("slice with incorrect length");
            i16::from_le_bytes(sized)
        })
        .collect();
    Ok(ret)
}

fn read_vu16<R: Read>(buf: &mut R) -> Result<Vec<u16>> {
    let size_ = read_u32(buf).expect("error reading key size");
    let size = usize::try_from(size_).unwrap();
    let chunk_size: usize = 2;
    let mut arr = vec![0u8; size * chunk_size];
    buf.read_exact(&mut arr).expect("error reading value");
    let ret = arr
        .chunks(chunk_size)
        .map(|chunk| {
            let sized: [u8; 2] = chunk.try_into().expect("slice with incorrect length");
            u16::from_le_bytes(sized)
        })
        .collect();
    Ok(ret)
}

fn read_vi32<R: Read>(buf: &mut R) -> Result<Vec<i32>> {
    let size_ = read_u32(buf).expect("error reading key size");
    let size = usize::try_from(size_).unwrap();
    let chunk_size: usize = 4;
    let mut arr = vec![0u8; size * chunk_size];
    buf.read_exact(&mut arr).expect("error reading value");
    let ret = arr
        .chunks(chunk_size)
        .map(|chunk| {
            let sized: [u8; 4] = chunk.try_into().expect("slice with incorrect length");
            i32::from_le_bytes(sized)
        })
        .collect();
    Ok(ret)
}

fn read_vu32<R: Read>(buf: &mut R) -> Result<Vec<u32>> {
    let size_ = read_u32(buf).expect("error reading key size");
    let size = usize::try_from(size_).unwrap();
    let chunk_size: usize = 4;
    let mut arr = vec![0u8; size * chunk_size];
    buf.read_exact(&mut arr).expect("error reading value");
    let ret = arr
        .chunks(chunk_size)
        .map(|chunk| {
            let sized: [u8; 4] = chunk.try_into().expect("slice with incorrect length");
            u32::from_le_bytes(sized)
        })
        .collect();
    Ok(ret)
}

fn read_vi64<R: Read>(buf: &mut R) -> Result<Vec<i64>> {
    let size_ = read_u32(buf).expect("error reading key size");
    let size = usize::try_from(size_).unwrap();
    let chunk_size: usize = 8;
    let mut arr = vec![0u8; size * chunk_size];
    buf.read_exact(&mut arr).expect("error reading value");
    let ret = arr
        .chunks(chunk_size)
        .map(|chunk| {
            let sized: [u8; 8] = chunk.try_into().expect("slice with incorrect length");
            i64::from_le_bytes(sized)
        })
        .collect();
    Ok(ret)
}

fn read_vu64<R: Read>(buf: &mut R) -> Result<Vec<u64>> {
    let size_ = read_u32(buf).expect("error reading key size");
    let size = usize::try_from(size_).unwrap();
    let chunk_size: usize = 8;
    let mut arr = vec![0u8; size * chunk_size];
    buf.read_exact(&mut arr).expect("error reading value");
    let ret = arr
        .chunks(chunk_size)
        .map(|chunk| {
            let sized: [u8; 8] = chunk.try_into().expect("slice with incorrect length");
            u64::from_le_bytes(sized)
        })
        .collect();
    Ok(ret)
}

fn read_vf32<R: Read>(buf: &mut R) -> Result<Vec<f32>> {
    let size_ = read_u32(buf).expect("error reading key size");
    let size = usize::try_from(size_).unwrap();
    let chunk_size: usize = 4;
    let mut arr = vec![0u8; size * chunk_size];
    buf.read_exact(&mut arr).expect("error reading value");
    let ret = arr
        .chunks(chunk_size)
        .map(|chunk| {
            let sized: [u8; 4] = chunk.try_into().expect("slice with incorrect length");
            f32::from_le_bytes(sized)
        })
        .collect();
    Ok(ret)
}

fn read_vf64<R: Read>(buf: &mut R) -> Result<Vec<f64>> {
    let size_ = read_u32(buf).expect("error reading key size");
    let size = usize::try_from(size_).unwrap();
    let chunk_size: usize = 8;
    let mut arr = vec![0u8; size * chunk_size];
    buf.read_exact(&mut arr).expect("error reading value");
    let ret = arr
        .chunks(chunk_size)
        .map(|chunk| {
            let sized: [u8; 8] = chunk.try_into().expect("slice with incorrect length");
            f64::from_le_bytes(sized)
        })
        .collect();
    Ok(ret)
}

fn read_u64<R: Read>(buf: &mut R) -> Result<u64> {
    let mut buffer = [0u8; 8];
    buf.read_exact(&mut buffer).expect("error reading value");
    let num = u64::from_le_bytes(buffer);
    Ok(num)
}

fn read_i64<R: Read>(buf: &mut R) -> Result<i64> {
    let mut buffer = [0u8; 8];
    buf.read_exact(&mut buffer).expect("error reading value");
    let num = i64::from_le_bytes(buffer);
    Ok(num)
}

fn read_u32<R: Read>(buf: &mut R) -> Result<u32> {
    let mut buffer = [0u8; 4];
    buf.read_exact(&mut buffer).expect("error reading value");
    let num = u32::from_le_bytes(buffer);
    Ok(num)
}

fn read_i32<R: Read>(buf: &mut R) -> Result<i32> {
    let mut buffer = [0u8; 4];
    buf.read_exact(&mut buffer).expect("error reading value");
    let num = i32::from_le_bytes(buffer);
    Ok(num)
}

fn read_u16<R: Read>(buf: &mut R) -> Result<u16> {
    let mut buffer = [0u8; 2];
    buf.read_exact(&mut buffer).expect("error reading value");
    let num = u16::from_le_bytes(buffer);
    Ok(num)
}

fn read_i16<R: Read>(buf: &mut R) -> Result<i16> {
    let mut buffer = [0u8; 2];
    buf.read_exact(&mut buffer).expect("error reading value");
    let num = i16::from_le_bytes(buffer);
    Ok(num)
}

fn read_char<R: Read>(buf: &mut R) -> Result<char> {
    let mut buffer = [0u8; 1];
    buf.read_exact(&mut buffer).expect("error reading value");
    Ok(buffer[0] as char)
}

fn read_u8<R: Read>(buf: &mut R) -> Result<u8> {
    let mut buffer = [0u8; 1];
    buf.read_exact(&mut buffer).expect("error reading value");
    Ok(buffer[0])
}

fn read_i8<R: Read>(buf: &mut R) -> Result<i8> {
    let mut buffer = [0u8; 1];
    buf.read_exact(&mut buffer).expect("error reading value");
    Ok(buffer[0] as i8)
}

fn read_bool<R: Read>(buf: &mut R) -> Result<bool> {
    let mut buffer = [0u8; 1];
    buf.read_exact(&mut buffer).expect("error reading value");
    Ok(buffer[0] != 0)
}

fn read_f32<R: Read>(buf: &mut R) -> Result<f32> {
    let mut buffer = [0u8; 4];
    buf.read_exact(&mut buffer).expect("error reading value");
    let num = f32::from_le_bytes(buffer);
    Ok(num)
}

fn read_f64<R: Read>(buf: &mut R) -> Result<f64> {
    let mut buffer = [0u8; 8];
    buf.read_exact(&mut buffer).expect("error reading value");
    let num = f64::from_le_bytes(buffer);
    Ok(num)
}

fn read_string<R: Read>(buf: &mut R) -> Result<String> {
    let size_ = read_u32(buf).expect("error reading string size");
    let size = usize::try_from(size_).unwrap();
    let mut arr = vec![0u8; size];
    buf.read_exact(&mut arr).expect("error reading string");
    let ret = String::from(std::str::from_utf8(&arr).expect("invalid utf-8 sequence"));
    Ok(ret)
}

pub fn read_schema<R: Read>(buf: &mut R) -> Result<Schema> {
    // read a size
    read_u32(buf).expect("error reading schema size");
    Ok(Schema::new(
        read_key(buf).expect("error reading schema size"),
        read_hash(buf).expect("error reading schema size"),
    ))
}

fn read_vstring<R: Read>(buf: &mut R) -> Result<Vec<String>> {
    let size = read_u32(buf).expect("error reading key size");
    let mut arr = Vec::new();
    for _ in 0..size {
        arr.push(read_string(buf).expect("error reading string in vector"));
    }
    Ok(arr)
}

fn read_key<R: Read>(buf: &mut R) -> Result<String> {
    let size = read_u8(buf).expect("error reading key size");
    let mut arr = vec![0u8; size.into()];
    buf.read_exact(&mut arr).expect("error reading key content");
    let key_name = String::from(std::str::from_utf8(&arr).expect("invalid utf-8 sequence"));
    Ok(key_name)
}

fn read_vhash<R: Read>(buf: &mut R) -> Result<Vec<Hash>> {
    let size = read_u32(buf).expect("error reading key size");
    let mut arr = Vec::new();
    for _ in 0..size {
        arr.push(read_hash(buf).unwrap());
    }
    Ok(arr)
}

pub fn read_hash<R: Read>(buf: &mut R) -> Result<Hash> {
    let mut hash = Hash::new();
    let nkeys = read_u32(buf).expect("error reading number of keys");
    for _ in 0..nkeys {
        let key = read_key(buf).expect("error reading key");
        let value_type = read_u32(buf).expect("error reading value type");
        let nattrs = read_u32(buf).expect("error reading number of attributes");
        let mut attrs = Attributes::new();
        for _ in 0..nattrs {
            let attr_key = read_key(buf).expect("error reading attribute  key");
            let attr_type = read_u32(buf).expect("error reading attribute type");
            let attr_value = read_value(buf, attr_type).unwrap();
            attrs.insert(&attr_key, attr_value);
        }
        let value = read_value(buf, value_type).unwrap();
        hash.insert_attrs(&key, value, attrs);
    }
    Ok(hash)
}

fn read_value<R: Read>(buf: &mut R, type_: u32) -> Result<HashValue> {
    match type_ {
        0 => Ok(HashValue::Bool(read_bool(buf).unwrap())),
        1 => Ok(HashValue::VectorBool(read_vbool(buf).unwrap())),
        2 => Ok(HashValue::Char(read_char(buf).unwrap())),
        3 => Ok(HashValue::VectorChar(read_vchar(buf).unwrap())),
        4 => Ok(HashValue::Int8(read_i8(buf).unwrap())),
        5 => Ok(HashValue::VectorInt8(read_vi8(buf).unwrap())),
        6 => Ok(HashValue::UInt8(read_u8(buf).unwrap())),
        7 => Ok(HashValue::VectorUInt8(read_vu8(buf).unwrap())),
        8 => Ok(HashValue::Int16(read_i16(buf).unwrap())),
        9 => Ok(HashValue::VectorInt16(read_vi16(buf).unwrap())),
        10 => Ok(HashValue::UInt16(read_u16(buf).unwrap())),
        11 => Ok(HashValue::VectorUInt16(read_vu16(buf).unwrap())),
        12 => Ok(HashValue::Int32(read_i32(buf).unwrap())),
        13 => Ok(HashValue::VectorInt32(read_vi32(buf).unwrap())),
        14 => Ok(HashValue::UInt32(read_u32(buf).unwrap())),
        15 => Ok(HashValue::VectorUInt32(read_vu32(buf).unwrap())),
        16 => Ok(HashValue::Int64(read_i64(buf).unwrap())),
        17 => Ok(HashValue::VectorInt64(read_vi64(buf).unwrap())),
        18 => Ok(HashValue::UInt64(read_u64(buf).unwrap())),
        19 => Ok(HashValue::VectorUInt64(read_vu64(buf).unwrap())),
        20 => Ok(HashValue::Float32(read_f32(buf).unwrap())),
        21 => Ok(HashValue::VectorFloat32(read_vf32(buf).unwrap())),
        22 => Ok(HashValue::Float64(read_f64(buf).unwrap())),
        23 => Ok(HashValue::VectorFloat64(read_vf64(buf).unwrap())),
        28 => Ok(HashValue::String(read_string(buf).unwrap())),
        29 => Ok(HashValue::VectorString(read_vstring(buf).unwrap())),
        30 => Ok(HashValue::Hash(read_hash(buf).unwrap())),
        31 => Ok(HashValue::VectorHash(read_vhash(buf).unwrap())),
        32 => Ok(HashValue::Schema(read_schema(buf).unwrap())),
        _ => panic!("Type {type_} not implemented"),
    }
}
