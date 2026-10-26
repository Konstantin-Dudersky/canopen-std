use crate::{Error, objects::ObjectDictionary};

use super::TpdoCommunication;

impl ObjectDictionary {
    pub fn read_tpdo_communication(&self, seq_number: u16) -> Result<TpdoCommunication, Error> {
        let index = 0x1800 + seq_number;

        let obj = self.get_object(index);

        let Some(obj) = obj else {
            return Ok(TpdoCommunication::default());
        };

        let v: TpdoCommunication = obj.try_into()?;

        Ok(v)
    }
}
