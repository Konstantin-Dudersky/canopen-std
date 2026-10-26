use crate::{Error, objects::ObjectDictionary};

use super::RpdoCommunication;

impl ObjectDictionary {
    pub fn read_rpdo_communication(&self, seq_number: u16) -> Result<RpdoCommunication, Error> {
        let index = RpdoCommunication::INDEX_BASE + seq_number;

        let obj = self.get_object(index);

        let Some(obj) = obj else {
            return Ok(RpdoCommunication::default());
        };

        let v: RpdoCommunication = obj.try_into()?;

        Ok(v)
    }
}
