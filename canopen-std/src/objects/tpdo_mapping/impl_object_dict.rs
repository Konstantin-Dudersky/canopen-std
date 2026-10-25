use crate::{Error, objects::ObjectDictionary};

use super::TpdoMapping;

impl ObjectDictionary {
    /// TPDO mapping
    pub fn read_tpdo_mapping(&self, sequence_number: u16) -> Result<TpdoMapping, Error> {
        let index = sequence_number + TpdoMapping::INDEX_BASE;

        let obj = self.get_object(index);

        let Some(obj) = obj else {
            return Ok(TpdoMapping::default());
        };

        let v: TpdoMapping = obj.try_into()?;

        Ok(v)
    }
}
