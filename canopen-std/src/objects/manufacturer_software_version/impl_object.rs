use crate::types::{AccessType, ByteString, Object, ObjectVar, Value};

use super::ManufacturerSoftwareVersion;

impl From<ManufacturerSoftwareVersion> for Object {
    fn from(value: ManufacturerSoftwareVersion) -> Self {
        let value = Value::VisibleString(ByteString::from_str(&value.name).unwrap());

        ObjectVar {
            index: ManufacturerSoftwareVersion::INDEX,
            name: ManufacturerSoftwareVersion::NAME.to_string(),
            access_type: AccessType::Ro,
            pdo_mapping: false,
            value,
        }
        .into()
    }
}
