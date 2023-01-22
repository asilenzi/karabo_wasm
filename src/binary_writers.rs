use std::io::{Cursor, Result, Write};

use crate::karabo_hash::{Hash, Schema, HashValue, get_hashtype};

fn write_string<W: Write>(buf: &mut W, s: &String) -> Result<usize> {
    let size = s.len() as u32;
    let mut size = buf.write(&size.to_le_bytes()).unwrap();
    size += buf.write(s.as_bytes()).unwrap();
    Ok(size)
}

fn write_key<W: Write>(buf: &mut W, s: &String) -> Result<usize> {
    let size = s.len() as u8;
    let mut size = buf.write(&[size]).unwrap();
    size += buf.write(s.as_bytes()).unwrap();
    Ok(size)
}


fn write_vec_u8<W: Write>(buf: &mut W, value: &Vec<u8>) -> Result<usize> {
    let vsize = value.len() as u32;
    let mut size = buf.write(&vsize.to_le_bytes()).unwrap();
    for el in value {
        size += buf.write(&el.to_le_bytes()).unwrap();
    }
    Ok(size)
}

fn write_vec_i8<W: Write>(buf: &mut W, value: &Vec<i8>) -> Result<usize> {
    let vsize = value.len() as u32;
    let mut size = buf.write(&vsize.to_le_bytes()).unwrap();
    for el in value {
        size += buf.write(&el.to_le_bytes()).unwrap();
    }
    Ok(size)
}

fn write_vec_i16<W: Write>(buf: &mut W, value: &Vec<i16>) -> Result<usize> {
    let vsize = value.len() as u32;
    let mut size = buf.write(&vsize.to_le_bytes()).unwrap();
    for el in value {
        size += buf.write(&el.to_le_bytes()).unwrap();
    }
    Ok(size)
}

fn write_vec_u16<W: Write>(buf: &mut W, value: &Vec<u16>) -> Result<usize> {
    let vsize = value.len() as u32;
    let mut size = buf.write(&vsize.to_le_bytes()).unwrap();
    for el in value {
        size += buf.write(&el.to_le_bytes()).unwrap();
    }
    Ok(size)
}

fn write_vec_i32<W: Write>(buf: &mut W, value: &Vec<i32>) -> Result<usize> {
    let vsize = value.len() as u32;
    let mut size = buf.write(&vsize.to_le_bytes()).unwrap();
    for el in value {
        size += buf.write(&el.to_le_bytes()).unwrap();
    }
    Ok(size)
}

fn write_vec_u32<W: Write>(buf: &mut W, value: &Vec<u32>) -> Result<usize> {
    let vsize = value.len() as u32;
    let mut size = buf.write(&vsize.to_le_bytes()).unwrap();
    for el in value {
        size += buf.write(&el.to_le_bytes()).unwrap();
    }
    Ok(size)
}

fn write_vec_i64<W: Write>(buf: &mut W, value: &Vec<i64>) -> Result<usize> {
    let vsize = value.len() as u32;
    let mut size = buf.write(&vsize.to_le_bytes()).unwrap();
    for el in value {
        size += buf.write(&el.to_le_bytes()).unwrap();
    }
    Ok(size)
}

fn write_vec_u64<W: Write>(buf: &mut W, value: &Vec<u64>) -> Result<usize> {
    let vsize = value.len() as u32;
    let mut size = buf.write(&vsize.to_le_bytes()).unwrap();
    for el in value {
        size += buf.write(&el.to_le_bytes()).unwrap();
    }
    Ok(size)
}

fn write_vec_f32<W: Write>(buf: &mut W, value: &Vec<f32>) -> Result<usize> {
    let vsize = value.len() as u32;
    let mut size = buf.write(&vsize.to_le_bytes()).unwrap();
    for el in value {
        size += buf.write(&el.to_le_bytes()).unwrap();
    }
    Ok(size)
}

fn write_vec_f64<W: Write>(buf: &mut W, value: &Vec<f64>) -> Result<usize> {
    let vsize = value.len() as u32;
    let mut size = buf.write(&vsize.to_le_bytes()).unwrap();
    for el in value {
        size += buf.write(&el.to_le_bytes()).unwrap();
    }
    Ok(size)
}

fn write_vstring<W: Write>(buf: &mut W, value: &Vec<String>) -> Result<usize> {
    let vsize = value.len() as u32;
    let mut size = buf.write(&vsize.to_le_bytes()).unwrap();
    for el in value {
        size += write_string(buf, el).unwrap();
    }
    Ok(size)
}

fn write_vhash<W: Write>(buf: &mut W, value: &Vec<Hash>) -> Result<usize>  {
    let vsize = value.len() as u32;
    let mut size = buf.write(&vsize.to_le_bytes()).unwrap();
    for el in value {
        size += write_hash(buf, &el).unwrap();
    }
    Ok(size)
}

fn write_schema<W: Write>(buf: &mut W, schema: &Schema) -> Result<usize>  {
    let mut c = Cursor::new(Vec::new());
    let mut size = write_string(&mut c, &schema.class_id).unwrap();
    size += write_hash(&mut c, &schema.hash).unwrap();
    let size32 = size as u32;
    let mut size2 = buf.write(&size32.to_le_bytes()).unwrap();
    size2 += buf.write(&c.into_inner()[..]).unwrap();
    Ok(size2)
}

fn write_value<W: Write>(buf: &mut W, value: &HashValue) -> Result<usize>  {
    match value {
        HashValue::Bool(x) => {
            if *x {
                return buf.write(&[1 as u8]);
            }
            return buf.write(&[0 as u8]);
        },
        HashValue::Char(x) => buf.write(&[*x]),
        HashValue::VectorChar(x) => write_vec_u8(buf, &x),
        HashValue::Int8(x) => buf.write(&[*x as u8]),
        HashValue::VectorInt8(x) => write_vec_i8(buf, &x),
        HashValue::UInt8(x) => buf.write(&[*x]),
        HashValue::VectorUInt8(x) => write_vec_u8(buf, &x),
        HashValue::Int16(x) => buf.write(&x.to_le_bytes()),
        HashValue::VectorInt16(x) => write_vec_i16(buf, &x),
        HashValue::UInt16(x) => buf.write(&x.to_le_bytes()),
        HashValue::VectorUInt16(x) => write_vec_u16(buf, &x),
        HashValue::Int32(x) => buf.write(&x.to_le_bytes()),
        HashValue::VectorInt32(x) => write_vec_i32(buf, &x),
        HashValue::UInt32(x) => buf.write(&x.to_le_bytes()),
        HashValue::VectorUInt32(x) => write_vec_u32(buf, &x),
        HashValue::Int64(x) => buf.write(&x.to_le_bytes()),
        HashValue::VectorInt64(x) => write_vec_i64(buf, &x),
        HashValue::UInt64(x) => buf.write(&x.to_le_bytes()),
        HashValue::VectorUInt64(x) => write_vec_u64(buf, &x),
        HashValue::Float32(x) => buf.write(&x.to_le_bytes()),
        HashValue::VectorFloat32(x) => write_vec_f32(buf, &x),
        HashValue::Float64(x) => buf.write(&x.to_le_bytes()),
        HashValue::VectorFloat64(x) => write_vec_f64(buf, &x),
        HashValue::String(x) => write_string(buf, &x),
        HashValue::VectorString(x) => write_vstring(buf, &x),
        HashValue::Hash(x) => write_hash(buf, &x),
        HashValue::VectorHash(x) => write_vhash(buf, &x),  // TODO: implement vector Hash
        HashValue::Schema(x) => write_schema(buf, &x),  // TODO: implement schema
    }
}

pub fn write_hash<W: Write>(buf: &mut W, hash: &Hash) -> Result<usize> {
    let nkeys = hash.len();
    let nkeys_32 = nkeys as u32;
    let mut size = buf.write(&nkeys_32.to_le_bytes()).unwrap();
    // TODO: figure out how to iterate a Hash
    for index in 0..nkeys {
        let node = hash.get_index(index).unwrap();
        size += write_key(buf, &node.key).unwrap();
        let type_ = get_hashtype(&node.value);
        size += buf.write(&type_.to_le_bytes()).unwrap();
        let nattrs = node.attrs.len();
        let nattrs_32 = nattrs as u32;
        size += buf.write(&nattrs_32.to_le_bytes()).unwrap();
        for attr_index in 0..nattrs {
            let attr = node.attrs.get_index(attr_index).unwrap();
            let attr_type = get_hashtype(&node.value);
            size += write_key(buf, &attr.key).unwrap();
            size += buf.write(&attr_type.to_le_bytes()).unwrap();
            size += write_value(buf, &attr.value).unwrap();
        }
        size += write_value(buf, &node.value).unwrap();
    }
    Ok(size)
}
