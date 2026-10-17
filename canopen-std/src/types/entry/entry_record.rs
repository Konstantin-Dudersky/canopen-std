use super::{AccessType, DataType, Entry};

pub struct EntryRecord {
    /// Descriptive name of the sub-index
    pub description: String,

    /// Data type classification
    pub data_type: DataType,

    pub access_type: AccessType,

    pub pdo_mapping: bool,

    pub default_value: String,
}

impl From<EntryRecord> for Entry {
    fn from(value: EntryRecord) -> Self {
        Self {
            description: Some(value.description),
            data_type: Some(value.data_type),
            access_type: value.access_type,
            pdo_mapping: value.pdo_mapping,
            value: value.default_value,
        }
    }
}
