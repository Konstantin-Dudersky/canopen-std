use bitvec::{field::BitField, order::Lsb0, view::BitView};

use crate::{
    Error,
    types::{AccessType, Entry, EntryRecord, Value},
};

#[derive(Debug)]
pub struct PdoMappingItem {
    pub index: u16,
    pub subindex: u8,
    pub length: u8,
}

impl PdoMappingItem {
    pub fn convert_items_to_entry_records(items: &[PdoMappingItem]) -> Vec<EntryRecord> {
        items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let subindex = i as u8 + 1;
                EntryRecord {
                    subindex,
                    description: format!("Mapping item #{}", subindex),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: item.into(),
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn convert_entries_to_items(entries: &[Entry]) -> Result<Vec<PdoMappingItem>, Error> {
        entries
            .iter()
            .map(|entry| entry.value.try_into())
            .collect::<Result<Vec<_>, Error>>()
    }
}

impl TryFrom<Value> for PdoMappingItem {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let raw = value.get_unsigned32()?;

        let bits = raw.view_bits::<Lsb0>();

        let index = bits[16..32].load_le::<u16>();
        let subindex = bits[8..16].load_le::<u8>();
        let length = bits[0..8].load_le::<u8>();

        Ok(PdoMappingItem {
            index,
            subindex,
            length,
        })
    }
}

impl From<PdoMappingItem> for Value {
    fn from(value: PdoMappingItem) -> Self {
        Self::from(&value)
    }
}

impl From<&PdoMappingItem> for Value {
    fn from(value: &PdoMappingItem) -> Self {
        let mut raw: u32 = 0;
        let bits = raw.view_bits_mut::<Lsb0>();
        bits[16..32].store_le(value.index);
        bits[8..16].store_le(value.subindex);
        bits[0..8].store_le(value.length);
        Value::Unsigned32(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let item: PdoMappingItem = PdoMappingItem {
            index: 0x1234,
            subindex: 0x56,
            length: 16,
        };
        let left: Value = item.into();
        let right = Value::Unsigned32(0x12345610);
        assert_eq!(left, right);
    }

    #[test]
    fn test_decode() {
        let value = Value::Unsigned32(0x20000010);
        let item: PdoMappingItem = value.try_into().unwrap();
        assert_eq!(item.index, 0x2000);
        assert_eq!(item.subindex, 0x00);
        assert_eq!(item.length, 16);
    }
}
