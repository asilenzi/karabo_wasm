#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, Cursor, Seek};

    use crate::karabo_hash::HashValue;
    #[test]
    fn read_from_file() {
        let mut read_buf = BufReader::new(File::open("./file.bin").unwrap());
        let hash = crate::binary_readers::read_hash(&mut read_buf).unwrap();
        let keys = hash.keys();
        assert_eq!(keys.len(), 27);
        let i = match hash["i8"] {
            HashValue::Int8(v) => v,
            _ => panic!("Unexpected type found for key i8"),
        };
        assert_eq!(i, -1i8);
    }

    #[test]
    fn test_hash_round() {
        let mut read_buf = BufReader::new(File::open("./file.bin").unwrap());
        let before = read_buf.stream_position().unwrap();
        let hash = crate::binary_readers::read_hash(&mut read_buf).unwrap();
        let after = read_buf.stream_position().unwrap();
        assert_eq!(hash.keys().len(), 27);

        let mut stream = Cursor::new(Vec::new());
        let size = crate::binary_writers::write_hash(&mut stream, &hash).unwrap();
        assert_eq!(size, (after - before) as usize);
        let vec = stream.into_inner();
        assert_eq!(size, vec.len());
        let mut read_buf = BufReader::new(vec.as_slice());
        let read_hash = crate::binary_readers::read_hash(&mut read_buf).unwrap();
        assert_eq!(read_hash.keys().len(), 27);
        assert_eq!(read_hash, hash);
    }
}
