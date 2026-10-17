use super::{AccessType, DataType, Value};

#[derive(Debug, Clone)]
pub struct Entry {
    /// Descriptive name of the sub-index (field only used for arrays, records and structures)
    pub description: Option<String>,

    /// Data type classification (field only used for records and structures)
    pub data_type: Option<DataType>,

    pub access_type: AccessType,

    pub pdo_mapping: bool,

    pub value: String,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            description: None,
            data_type: None,
            access_type: AccessType::Ro,
            pdo_mapping: false,
            value: "".to_string(),
        }
    }
}
