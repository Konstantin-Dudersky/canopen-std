use super::{AccessType, DataType, Entry, Object};

pub struct ObjectVar {
    pub index: u16,
    pub name: String,
    pub data_type: DataType,
    pub access_type: AccessType,
    pub pdo_mapping: bool,
    pub default_value: String,
}

impl From<ObjectVar> for Object {
    fn from(value: ObjectVar) -> Self {
        Self::Var {
            index: value.index,
            name: value.name,
            data_type: value.data_type,
            entry: Entry {
                description: None,
                data_type: None,
                access_type: value.access_type,
                pdo_mapping: value.pdo_mapping,
                value: value.default_value,
            },
        }
    }
}
