use crate::Error;
use crate::objects::ObjectDictionary;

use super::CommCyclePeriod;

impl ObjectDictionary {
    pub fn read_comm_cycle_period(&self) -> Result<CommCyclePeriod, Error> {
        let obj = self.get_object(CommCyclePeriod::INDEX);

        let Some(obj) = obj else {
            let v = CommCyclePeriod::default();
            return Ok(v);
        };

        let v: CommCyclePeriod = obj.try_into()?;
        Ok(v)
    }
}
