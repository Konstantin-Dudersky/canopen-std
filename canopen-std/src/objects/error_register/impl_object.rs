use bitvec::{order::Lsb0, view::BitView};

use crate::{
    Error,
    types::{AccessType, Object, ObjectVar, Value},
};

use super::ErrorRegister;

impl From<ErrorRegister> for Object {
    fn from(value: ErrorRegister) -> Self {
        let mut raw: u8 = 0;
        let bits = raw.view_bits_mut::<Lsb0>();
        bits.set(0, value.generic_error);
        bits.set(1, value.current);
        bits.set(2, value.voltage);
        bits.set(3, value.temperature);
        bits.set(4, value.communication_error);
        bits.set(5, value.device_profile_specific);
        bits.set(7, value.manufacturer_specific);

        Object::from(ObjectVar {
            index: ErrorRegister::INDEX,
            name: ErrorRegister::NAME.to_string(),
            access_type: AccessType::Ro,
            pdo_mapping: false,
            value: Value::Unsigned8(raw),
        })
    }
}

impl TryFrom<&Object> for ErrorRegister {
    type Error = Error;

    fn try_from(value: &Object) -> Result<Self, Self::Error> {
        let entry = value.get_entry(0).ok_or(Error::EntryNotFound {
            index: ErrorRegister::INDEX,
            subindex: 0,
        })?;

        let value = entry.value.get_unsigned8()?;
        let bits = value.view_bits::<Lsb0>();

        Ok(Self {
            generic_error: bits[0],
            current: bits[1],
            voltage: bits[2],
            temperature: bits[3],
            communication_error: bits[4],
            device_profile_specific: bits[5],
            manufacturer_specific: bits[7],
        })
    }
}
