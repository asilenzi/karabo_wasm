#[cfg(test)]
mod tests {
    use crate::attributes::Attributes;
    use crate::binary_readers::read_hash;
    use crate::binary_writers::write_hash;
    use crate::hash::Hash;
    use crate::types::HashValue;
    use std::fs::File;
    use std::io::{BufReader, Cursor, Seek};

    #[test]
    fn read_from_file() {
        let mut read_buf = BufReader::new(File::open("./file.bin").unwrap());
        let hash = read_hash(&mut read_buf).unwrap();
        let keys = hash.keys();
        assert_eq!(keys.len(), 27);
        let i = hash["i8"].as_i8().unwrap();
        assert_eq!(i, -1i8);
        let attrs = hash.get_attributes("i8").unwrap();
        assert!(attrs.is_empty());
        assert!(attrs.get("BOBIsNotAnAttribute").is_none());
        let i = hash["u8"].as_u8().unwrap();
        assert_eq!(i, 250u8);
        let i = hash["i16"].as_i16().unwrap();
        assert_eq!(i, -200i16);
        let i = hash["u16"].as_u16().unwrap();
        assert_eq!(i, 599u16);
        let i = hash["i32"].as_i32().unwrap();
        assert_eq!(i, 12i32);
        let i = hash["u32"].as_u32().unwrap();
        assert_eq!(i, 12u32);
        let i = hash["i64"].as_i64().unwrap();
        assert_eq!(i, -12i64);
        let i = hash["u64"].as_u64().unwrap();
        assert_eq!(i, 12u64);
        let i = hash["f32"].as_f32().unwrap();
        assert_eq!(i, 12f32);
        let i = hash["f64"].as_f64().unwrap();
        assert_eq!(i, 200f64);
        let i = hash["node.f64"].as_f64().unwrap();
        assert_eq!(i, 200f64);
        let i = hash["schema"].as_schema().unwrap();
        assert_eq!(i.class_id, "SimpleSchema");
        let _outstr = format!("{}", hash);
        let hash = hash["node"].as_hash().unwrap();
        let i = hash["i8"].as_i8().unwrap();
        assert_eq!(i, -1i8);
        let attrs = hash.get_attributes("i8").unwrap();
        assert!(attrs.is_empty());
        assert!(attrs.get("BOBIsNotAnAttribute").is_none());
        let i = hash["u8"].as_u8().unwrap();
        assert_eq!(i, 250u8);
        let i = hash["i16"].as_i16().unwrap();
        assert_eq!(i, -200i16);
        let i = hash["u16"].as_u16().unwrap();
        assert_eq!(i, 599u16);
        let i = hash["i32"].as_i32().unwrap();
        assert_eq!(i, 12i32);
        let i = hash["u32"].as_u32().unwrap();
        assert_eq!(i, 12u32);
        let i = hash["i64"].as_i64().unwrap();
        assert_eq!(i, -12i64);
        let i = hash["u64"].as_u64().unwrap();
        assert_eq!(i, 12u64);
        let i = hash["f32"].as_f32().unwrap();
        assert_eq!(i, 12f32);
        let i = hash["f64"].as_f64().unwrap();
        assert_eq!(i, 200f64);
        assert_eq!(hash.len(), 24);
    }

    #[test]
    fn test_hash_round() {
        let mut read_buf = BufReader::new(File::open("./file.bin").unwrap());
        let before = read_buf.stream_position().unwrap();
        let hash = read_hash(&mut read_buf).unwrap();
        let after = read_buf.stream_position().unwrap();
        assert_eq!(hash.keys().len(), 27);

        let mut stream = Cursor::new(Vec::new());
        let size = write_hash(&mut stream, &hash).unwrap();
        assert_eq!(size, (after - before) as usize);
        let vec = stream.into_inner();
        assert_eq!(size, vec.len());
        let mut read_buf = BufReader::new(vec.as_slice());
        let read_hash = read_hash(&mut read_buf).unwrap();
        assert_eq!(read_hash.keys().len(), 27);
        assert_eq!(read_hash, hash);
    }

    #[test]
    fn test_hash_round_chars() {
        let mut hash = Hash::new();
        assert!(hash.is_empty());
        hash.insert("char", HashValue::Char('a'));
        let vc = br#"abcdefghijklmno"#.to_vec().iter().map(|&e| e as char).collect();
        hash.insert("vchar", HashValue::VectorChar(vc));
        hash.insert("bool", HashValue::Bool(false));
        assert_eq!(hash.keys().len(), 3);

        let mut stream = Cursor::new(Vec::new());
        let size = write_hash(&mut stream, &hash).unwrap();
        let vec = stream.into_inner();
        assert_eq!(size, vec.len());
        let mut read_buf = BufReader::new(vec.as_slice());
        let read_hash = read_hash(&mut read_buf).unwrap();
        assert_eq!(read_hash.keys().len(), 3);
        assert_eq!(read_hash, hash);
        let mut iter = hash.into_iter();
        assert_eq!("char", iter.next().unwrap().key);
        assert_eq!("vchar", iter.next().unwrap().key);
        assert_eq!("bool", iter.next().unwrap().key);
    }

    #[test]
    fn test_equal() {
        // same key different type
        let mut hash = Hash::new();
        hash.insert("a", HashValue::Char('a'));
        let mut hash2 = Hash::new();
        assert!(hash != hash2);
        hash2.insert("a", HashValue::UInt16(42));
        assert!(hash != hash2);
        // same key same type different attribute
        let mut hash = Hash::new();
        let mut attrs = Attributes::new();
        attrs.insert("attr1", HashValue::UInt32(0));
        hash.insert_attrs("a", HashValue::Char('a'), attrs);
        let mut hash2 = Hash::new();
        let mut attrs2 = Attributes::new();
        attrs2.insert("attr2", HashValue::UInt32(0));
        hash2.insert_attrs("a", HashValue::Char('a'), attrs2);
        assert!(hash != hash2);
        let mut hash2 = Hash::new();
        hash2.insert("a", HashValue::Char('a'));
        assert!(hash != hash2);
        // same key same type same attribute
        // different attribute type
        let mut hash = Hash::new();
        let mut attrs = Attributes::new();
        attrs.insert("attr1", HashValue::UInt32(0));
        hash.insert_attrs("a", HashValue::Char('a'), attrs);
        let mut hash2 = Hash::new();
        let mut attrs2 = Attributes::new();
        attrs2.insert("attr1", HashValue::UInt16(0));
        hash2.insert_attrs("a", HashValue::Char('a'), attrs2);
        assert!(hash != hash2);
    }

    #[test]
    fn test_hash_getter_setter() {
        let mut hash = Hash::new();
        assert!(hash.is_empty());
        hash.insert("a", HashValue::UInt8(10));
        let val = hash["a"].as_u8().unwrap();
        assert_eq!(val, 10u8);
        assert_eq!(hash.len(), 1);
        hash.insert("a", HashValue::Int8(-10));
        let val = hash["a"].as_i8().unwrap();
        assert_eq!(val, -10i8);
        assert_eq!(hash.len(), 1);
    }

    #[test]
    #[should_panic]
    fn test_invalid_key() {
        let hash = Hash::new();
        let _ = hash["missing_key"];
    }
}
