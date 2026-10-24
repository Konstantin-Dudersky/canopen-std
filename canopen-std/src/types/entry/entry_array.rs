use super::{AccessType, Entry, Value};

pub struct EntryArray {
    pub subindex: u8,

    /// Descriptive name of the sub-index
    pub description: String,

    pub access_type: AccessType,

    pub pdo_mapping: bool,

    pub value: Value,
}

impl From<EntryArray> for Entry {
    fn from(value: EntryArray) -> Self {
        Self {
            subindex: value.subindex,
            description: Some(value.description),
            access_type: value.access_type,
            pdo_mapping: value.pdo_mapping,
            value: value.value,
        }
    }
}
