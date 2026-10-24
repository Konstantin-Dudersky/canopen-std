use crate::types::{AccessType, EntryRecord, Object, ObjectRecord, Value};

use super::IdentityObject;

impl From<IdentityObject> for Object {
    fn from(value: IdentityObject) -> Self {
        ObjectRecord {
            index: IdentityObject::INDEX,
            name: IdentityObject::NAME.to_string(),
            entries: vec![
                EntryRecord {
                    subindex: 1,
                    description: "Vendor-ID".to_string(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: Value::Unsigned32(value.vendor_id),
                },
                EntryRecord {
                    subindex: 2,
                    description: "Product code".to_string(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: Value::Unsigned32(value.product_code),
                },
                EntryRecord {
                    subindex: 3,
                    description: "Revision number".to_string(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: Value::Unsigned32(value.revision_number),
                },
                EntryRecord {
                    subindex: 4,
                    description: "Serial number".to_string(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: Value::Unsigned32(value.serial_number),
                },
            ],
        }
        .into()
    }
}
