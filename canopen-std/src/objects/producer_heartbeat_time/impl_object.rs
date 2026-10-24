use std::time::Duration;

use crate::{
    Error,
    types::{AccessType, Object, ObjectVar, Value},
};

use super::ProducerHeartbeatTime;

impl From<ProducerHeartbeatTime> for Object {
    fn from(value: ProducerHeartbeatTime) -> Self {
        let value = Value::Unsigned16(value.heartbeat_time.as_millis() as u16);
        ObjectVar {
            index: ProducerHeartbeatTime::INDEX,
            name: ProducerHeartbeatTime::NAME.into(),
            access_type: AccessType::Rw,
            pdo_mapping: false,
            value,
        }
        .into()
    }
}

impl TryFrom<&Object> for ProducerHeartbeatTime {
    type Error = Error;

    fn try_from(value: &Object) -> Result<Self, Self::Error> {
        let entry = value.get_entry(0).ok_or(Error::EntryNotFound {
            index: Self::INDEX,
            subindex: 0,
        })?;

        let value = entry.value.get_unsigned16()?;
        let value = Duration::from_millis(value as u64);

        Ok(ProducerHeartbeatTime {
            heartbeat_time: value,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::types::{DataType, Entry};

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        let obj = ProducerHeartbeatTime {
            heartbeat_time: Duration::from_millis(1234),
        };

        let left: Object = obj.into();

        let right = Object::Var {
            index: 0x1017,
            name: "Producer heartbeat time".into(),
            data_type: DataType::Unsigned16,
            entry: Entry {
                subindex: 0,
                description: Some("Producer heartbeat time".into()),
                access_type: AccessType::Rw,
                pdo_mapping: false,
                value: Value::Unsigned16(1234),
            },
        };

        assert_eq!(left, right);
    }
}
