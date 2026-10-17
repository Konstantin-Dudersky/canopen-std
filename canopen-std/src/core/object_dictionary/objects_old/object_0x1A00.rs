#![allow(non_snake_case)]

use super::{common::PdoMappingItem, Address, Error, ObjectDictionary};

/// TPDO mapping
#[derive(Debug)]
pub struct TpdoMapping {
    pub items: Vec<PdoMappingItem>,
}

impl ObjectDictionary {
    /// TPDO mapping
    #[allow(non_snake_case)]
    pub fn read_0x1A00(&self, tpdo_index: u16) -> Result<TpdoMapping, Error> {
        let index = 0x1A00 + tpdo_index;

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

        Ok(TpdoMapping { items })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::types::Value;

    use super::*;

    #[test]
    fn test_decode() {
        let value = Value::Unsigned32(0x20000010);
        let item: PdoMappingItem = value.try_into().unwrap();
        assert_eq!(item.index, 0x2000);
        assert_eq!(item.subindex, 0x00);
        assert_eq!(item.length, 16);
    }
}
