use std::time::Duration;

use crate::{
    Error,
    types::{AccessType, Object, ObjectVar, Value},
};

use super::HighResolutionTimeStamp;

impl From<HighResolutionTimeStamp> for Object {
    fn from(value: HighResolutionTimeStamp) -> Self {
        let value = Value::Unsigned32(value.time.as_micros() as u32);

        ObjectVar {
            index: HighResolutionTimeStamp::INDEX,
            name: HighResolutionTimeStamp::NAME.to_string(),
            access_type: AccessType::Rw,
            pdo_mapping: false,
            value,
        }
        .into()
    }
}

impl TryFrom<&Object> for HighResolutionTimeStamp {
    type Error = Error;

    fn try_from(obj: &Object) -> Result<Self, Self::Error> {
        let entry = obj.get_entry(0).ok_or(Error::EntryNotFound {
            index: Self::INDEX,
            subindex: 0,
        })?;

        let time = entry.value.get_unsigned32()?;
        Ok(HighResolutionTimeStamp {
            time: Duration::from_micros(time as u64),
        })
    }
}
