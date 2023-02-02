#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, Cursor};

    use crate::karabo_hash::HashValue;
    #[test]
    fn read_from_file() {
        let mut read_buf = BufReader::new(File::open("./file.bin").unwrap());
        let hash = crate::binary_readers::read_hash(&mut read_buf).unwrap();
        let keys = hash.keys();
        assert_eq!(keys.len(), 25);
        let i = match hash["i8"] {
            HashValue::Int8(v) => v,
            _ => panic!("Unexpected type found for key i8"),
        };
        assert_eq!(i, -1i8);
    }

    #[test]
    fn test_hash_round() {
        let mut read_buf = BufReader::new(File::open("./file.bin").unwrap());
        let hash = crate::binary_readers::read_hash(&mut read_buf).unwrap();
        assert_eq!(hash.keys().len(), 25);

        let mut stream = Cursor::new(Vec::new());
        let size = crate::binary_writers::write_hash(&mut stream, &hash).unwrap();
        assert_eq!(size, 1513);
        let vec = stream.into_inner();
        assert_eq!(size, vec.len());
        let mut read_buf = BufReader::new(vec.as_slice());
        let read_hash = crate::binary_readers::read_hash(&mut read_buf).unwrap();
        assert_eq!(read_hash.keys().len(), 25);

        let keys = hash.keys();
        let read_keys = read_hash.keys();
        assert_eq!(read_keys.len(), keys.len());
        for (i, item) in keys.iter().enumerate() {
            assert_eq!(*item, read_keys[i]);
        }
    }
}