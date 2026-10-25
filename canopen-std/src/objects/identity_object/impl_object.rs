use crate::{
    Error,
    types::{AccessType, EntryRecord, Object, ObjectRecord, Value},
};

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

impl TryFrom<&Object> for IdentityObject {
    type Error = Error;

    fn try_from(obj: &Object) -> Result<Self, Self::Error> {
        let entry_vendor_id = obj.get_entry(1).ok_or(Error::EntryNotFound {
            index: Self::INDEX,
            subindex: 1,
        })?;
        let vendor_id = entry_vendor_id.value.get_unsigned32()?;

        let entry_product_code = obj.get_entry(2).ok_or(Error::EntryNotFound {
            index: Self::INDEX,
            subindex: 2,
        })?;
        let product_code = entry_product_code.value.get_unsigned32()?;

        let entry_revision_number = obj.get_entry(3).ok_or(Error::EntryNotFound {
            index: Self::INDEX,
            subindex: 3,
        })?;
        let revision_number = entry_revision_number.value.get_unsigned32()?;

        let entry_serial_number = obj.get_entry(4).ok_or(Error::EntryNotFound {
            index: Self::INDEX,
            subindex: 4,
        })?;
        let serial_number = entry_serial_number.value.get_unsigned32()?;

        Ok(Self {
            vendor_id,
            product_code,
            revision_number,
            serial_number,
        })
    }
}
