use super::{EntryArray, Object};

pub struct ObjectArray {
    pub index: u16,
    pub name: String,
    pub entries: Vec<EntryArray>,
}

impl From<ObjectArray> for Object {
    fn from(value: ObjectArray) -> Self {
        Self::Array {
            index: value.index,
            name: value.name,
            entries: value
                .entries
                .into_iter()
                .map(|entry| entry.into())
                .collect(),
        }
    }
}
