use crate::hash::Hash;

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
