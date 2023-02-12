#[cfg(test)]
mod tests {
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
        let i = match &hash["schema"] {
            HashValue::Schema(s) => s,
            _ => panic!("Unexpected type found for key schema"),
        };
        assert_eq!(i.class_id, "SimpleSchema");
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
}
