use crate::{Error, objects::ObjectDictionary};

use super::ProducerHeartbeatTime;

impl ObjectDictionary {
    pub fn read_producer_heartbeat_time(&self) -> Result<ProducerHeartbeatTime, Error> {
        let obj = self.get_object(ProducerHeartbeatTime::INDEX);
        let Some(obj) = obj else {
            return Ok(ProducerHeartbeatTime::default());
        };

        let v: ProducerHeartbeatTime = obj.try_into()?;

        Ok(v)
    }
}
