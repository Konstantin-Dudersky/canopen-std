mod rpdo_mapping;

pub use rpdo_mapping::RpdoMapping;

use super::{Address, Error, ObjectDictionary};

impl ObjectDictionary {
    /// RPDO mapping
    #[allow(non_snake_case)]
    pub fn read_rpdo_mapping(&self, sequence_number: u16) -> Result<RpdoMapping, Error> {
        let index = sequence_number + 0x1600;

        let address = Address::new(index, 0);
        let entry = self.entries.get(&address);
        let Some(entry) = entry else {
            return Err(Error::ObjectNotFound);
        };
        let number = entry.value.get_unsigned8()?;

        let mut items = Vec::with_capacity(number as usize);
        for i in 1..=number {
            let address = Address::new(index, i);
            let entry = self.entries.get(&address);
            let Some(entry) = entry else {
                return Err(Error::ObjectNotFound);
            };
            let item = entry.value.try_into()?;
            items.push(item);
        }

        Ok(RpdoMapping {
            sequence_number,
            items,
        })
    }
}
