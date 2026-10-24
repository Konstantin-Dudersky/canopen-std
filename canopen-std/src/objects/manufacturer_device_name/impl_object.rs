use crate::types::{AccessType, ByteString, Object, ObjectVar, Value};

use super::ManufacturerDeviceName;

impl From<ManufacturerDeviceName> for Object {
    fn from(value: ManufacturerDeviceName) -> Self {
        let value = Value::VisibleString(ByteString::from_str(&value.name).unwrap());

        ObjectVar {
            index: ManufacturerDeviceName::INDEX,
            name: ManufacturerDeviceName::NAME.to_string(),
            access_type: AccessType::Ro,
            pdo_mapping: false,
            value,
        }
        .into()
    }
}
