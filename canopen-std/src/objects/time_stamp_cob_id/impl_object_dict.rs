use crate::{Error, objects::ObjectDictionary};

use super::TimeStampCobId;

impl ObjectDictionary {
    pub fn read_time_stamp_cob_id(&self) -> Result<TimeStampCobId, Error> {
        let obj = self.get_object(TimeStampCobId::INDEX);

        let Some(obj) = obj else {
            let v = TimeStampCobId::default();
            return Ok(v);
        };

        let v: TimeStampCobId = obj.try_into()?;
        Ok(v)
    }
}
