use crate::{
    Error,
    types::{AccessType, Object, ObjectVar, Value},
};

use super::DeviceType;

impl From<DeviceType> for Object {
    fn from(value: DeviceType) -> Self {
        ObjectVar {
            index: DeviceType::INDEX,
            name: DeviceType::NAME.to_string(),
            access_type: AccessType::Ro,
            pdo_mapping: false,
            value: Value::Unsigned32(value.device_type),
        }
        .into()
    }
}

impl TryFrom<&Object> for DeviceType {
    type Error = Error;

    fn try_from(value: &Object) -> Result<Self, Self::Error> {
        let entry = value.get_entry(0).ok_or(Error::EntryNotFound {
            index: DeviceType::INDEX,
            subindex: 0,
        })?;

        let device_type = entry.value.get_unsigned32()?;

        Ok(DeviceType { device_type })
    }
}
