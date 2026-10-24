use super::{AccessType, Entry, Object, Value};

pub struct ObjectVar {
    pub index: u16,
    pub name: String,
    pub access_type: AccessType,
    pub pdo_mapping: bool,
    pub value: Value,
}

impl From<ObjectVar> for Object {
    fn from(v: ObjectVar) -> Self {
        Self::Var {
            index: v.index,
            name: v.name.clone(),
            data_type: v.value.data_type(),
            entry: Entry {
                subindex: 0,
                description: Some(v.name),
                access_type: v.access_type,
                pdo_mapping: v.pdo_mapping,
                value: v.value,
            },
        }
    }
}
