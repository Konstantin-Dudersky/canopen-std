use crate::{Error, objects::ObjectDictionary};

use super::SyncMessageCobId;

impl ObjectDictionary {
    pub fn read_sync_message_cob_id(&self) -> Result<SyncMessageCobId, Error> {
        let obj = self.get_object(SyncMessageCobId::INDEX);
        let Some(obj) = obj else {
            return Ok(SyncMessageCobId::default());
        };

        let v: SyncMessageCobId = obj.try_into()?;

        Ok(v)
    }
}
