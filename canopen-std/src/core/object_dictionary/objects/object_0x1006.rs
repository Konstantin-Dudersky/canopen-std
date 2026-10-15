use std::time::Duration;

use super::{Address, Error, ObjectDictionary};

/// Communication cycle period
pub struct Object0x1006 {
    pub period: Duration,
}

impl ObjectDictionary {
    pub fn read_0x1006(&self) -> Result<Object0x1006, Error> {
        let address = Address::new(0x1006, 0);
        let entry = self.entries.get(&address);
        let Some(entry) = entry else {
            return Ok(Object0x1006 {
                period: Duration::from_millis(0),
            });
        };

        let raw = entry.value.get_unsigned32()?;
        let period = Duration::from_micros(raw as u64);
        let obj = Object0x1006 { period };

        Ok(obj)
    }
}
