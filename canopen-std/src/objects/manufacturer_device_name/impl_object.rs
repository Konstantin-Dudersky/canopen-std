use crate::{
    Error,
    types::{AccessType, ByteString, Object, ObjectVar, Value},
};

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

impl TryFrom<&Object> for ManufacturerDeviceName {
    type Error = Error;

    fn try_from(obj: &Object) -> Result<Self, Self::Error> {
        let entry = obj.get_entry(0).ok_or(Error::EntryNotFound {
            index: Self::INDEX,
            subindex: 0,
        })?;

        let name = entry
            .value
            .get_visible_string()?
            .as_str()
            .unwrap()
            .to_string();

        Ok(Self { name })
    }
}
