use super::{DataType, EntryArray, Object};

pub struct ObjectArray {
    pub index: u16,
    pub name: String,
    pub data_type: DataType,
    pub entries: Vec<EntryArray>,
}

impl From<ObjectArray> for Object {
    fn from(value: ObjectArray) -> Self {
        Self::Array {
            index: value.index,
            name: value.name,
            data_type: value.data_type,
            entries: value.entries.into_iter().map(|e| e.into()).collect(),
        }
    }
}
