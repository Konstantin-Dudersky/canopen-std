use crate::{Error, objects::ObjectDictionary};

use super::ConsumerHeartbeatTime;

impl ObjectDictionary {
    pub fn read_consumer_heartbeat_time(&self) -> Result<ConsumerHeartbeatTime, Error> {
        let obj = self.get_object(ConsumerHeartbeatTime::INDEX);
        let Some(obj) = obj else {
            return Ok(ConsumerHeartbeatTime { items: vec![] });
        };

        let v: ConsumerHeartbeatTime = obj.try_into()?;

        Ok(v)
    }
}
