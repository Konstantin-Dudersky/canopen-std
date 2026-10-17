use std::time::Duration;

use crate::{
    core::object_dictionary::{AccessType, Entry},
    types::Value,
};

use super::{Address, Error, ObjectDictionary};

/// 0x1017 - Producer heartbeat time
pub struct ProducerHeartbeatTime {
    pub heartbeat_time: Duration,
    pub heartbeat_time_access: AccessType,
}

impl ProducerHeartbeatTime {
    pub fn entries(self) -> Vec<Entry> {
        let entry = Entry {
            address: Address::new(0x1017, 0),
            value: Value::Unsigned16(self.heartbeat_time.as_millis() as u16),
            access: AccessType::Rw,
        };
        vec![entry]
    }
}

impl ObjectDictionary {
    pub fn read_object_0x1017(&self) -> Result<ProducerHeartbeatTime, Error> {
        let address = Address::new(0x1017, 0);
        let entry = self.entries.get(&address);

        let Some(entry) = entry else {
            return Ok(ProducerHeartbeatTime {
                heartbeat_time: Duration::from_millis(0),
                heartbeat_time_access: AccessType::Rw,
            });
        };

        let value = entry.value.get_unsigned16()?;
        let heartbeat_time = Duration::from_millis(value as u64);
        let heartbeat_time_access = entry.access;

        Ok(ProducerHeartbeatTime {
            heartbeat_time,
            heartbeat_time_access,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        let obj = ProducerHeartbeatTime {
            heartbeat_time: Duration::from_millis(1234),
            heartbeat_time_access: AccessType::Rw,
        };

        let left: Vec<Entry> = obj.entries();

        let right = vec![Entry {
            address: Address::new(0x1017, 0),
            value: Value::Unsigned16(1234),
            access: AccessType::Rw,
        }];

        assert_eq!(left, right);
    }
}
