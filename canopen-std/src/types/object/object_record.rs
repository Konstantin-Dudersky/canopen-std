use super::{DataType, EntryRecord, Object};

pub struct ObjectRecord {
    pub index: u16,
    pub name: String,
    pub data_type: DataType,
    pub entries: Vec<EntryRecord>,
}

impl From<ObjectRecord> for Object {
    fn from(value: ObjectRecord) -> Self {
        Self::Record {
            index: value.index,
            name: value.name,
            data_type: value.data_type,
            entries: value.entries.into_iter().map(|e| e.into()).collect(),
        }
    }
}
