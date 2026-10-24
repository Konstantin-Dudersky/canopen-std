use crate::{Error, objects::ObjectDictionary};

use super::SyncCounterOverflow;

impl ObjectDictionary {
    pub fn read_sync_counter_overflow(&self) -> Result<SyncCounterOverflow, Error> {
        let obj = self.get_object(SyncCounterOverflow::INDEX);
        let Some(obj) = obj else {
            return Ok(SyncCounterOverflow::default());
        };

        let v: SyncCounterOverflow = obj.try_into()?;

        Ok(v)
    }
}
