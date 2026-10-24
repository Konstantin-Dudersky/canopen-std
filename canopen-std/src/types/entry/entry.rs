use super::{AccessType, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub subindex: u8,

    /// Descriptive name of the sub-index (field only used for arrays, records and structures)
    pub description: Option<String>,

    /// Data type classification (field only used for records and structures)
    pub access_type: AccessType,

    pub pdo_mapping: bool,

    pub value: Value,
}
