use super::{EntryRecord, Object};

pub struct ObjectRecord {
    pub index: u16,
    pub name: String,
    pub entries: Vec<EntryRecord>,
}

impl From<ObjectRecord> for Object {
    fn from(v: ObjectRecord) -> Self {
        Self::Record {
            index: v.index,
            name: v.name,
            entries: v.entries.into_iter().map(|entry| entry.into()).collect(),
        }
    }
}
