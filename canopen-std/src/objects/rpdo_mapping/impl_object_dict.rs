use crate::{Error, objects::ObjectDictionary};

use super::RpdoMapping;

impl ObjectDictionary {
    /// RPDO mapping
    #[allow(non_snake_case)]
    pub fn read_rpdo_mapping(&self, sequence_number: u16) -> Result<RpdoMapping, Error> {
        let index = sequence_number + RpdoMapping::INDEX_BASE;

        let obj = self.get_object(index);

        let Some(obj) = obj else {
            return Ok(RpdoMapping::default());
        };

        let v: RpdoMapping = obj.try_into()?;

        Ok(v)
    }
}
