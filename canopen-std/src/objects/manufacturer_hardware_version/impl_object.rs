use crate::types::{AccessType, ByteString, Object, ObjectVar, Value};

use super::ManufacturerHardwareVersion;

impl From<ManufacturerHardwareVersion> for Object {
    fn from(value: ManufacturerHardwareVersion) -> Self {
        let value = Value::VisibleString(ByteString::from_str(&value.name).unwrap());

        ObjectVar {
            index: ManufacturerHardwareVersion::INDEX,
            name: ManufacturerHardwareVersion::NAME.to_string(),
            access_type: AccessType::Ro,
            pdo_mapping: false,
            value,
        }
        .into()
    }
}
