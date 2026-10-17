use super::{AccessType, Entry};

pub struct EntryArray {
    /// Descriptive name of the sub-index
    pub description: String,

    pub access_type: AccessType,

    pub pdo_mapping: bool,

    pub default_value: String,
}

impl From<EntryArray> for Entry {
    fn from(value: EntryArray) -> Self {
        Self {
            description: Some(value.description),
            data_type: None,
            access_type: value.access_type,
            pdo_mapping: value.pdo_mapping,
            value: value.default_value,
        }
    }
}
