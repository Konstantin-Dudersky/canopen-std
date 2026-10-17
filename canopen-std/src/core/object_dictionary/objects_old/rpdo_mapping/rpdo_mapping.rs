use crate::{types::Value, AccessType, Entry};

use super::super::{
    common::{ObjectTrait, PdoMappingItem},
    Address,
};

/// 0x1600..=0x17FF - RPDO mapping parameter
#[derive(Debug)]
pub struct RpdoMapping {
    pub sequence_number: u16,
    pub items: Vec<PdoMappingItem>,
}

impl ObjectTrait for RpdoMapping {
    fn entries(self) -> Vec<crate::Entry> {
        let index = 0x1600 + self.sequence_number;

        let len = self.items.len() as u8;

        let mut entries: Vec<Entry> = self
            .items
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let subindex = i as u8 + 1;
                Entry {
                    address: Address::new(index, subindex),
                    value: item.into(),
                    access: AccessType::Ro,
                }
            })
            .collect::<Vec<_>>();

        entries.push(Entry {
            address: Address::new(index, 0),
            value: Value::Unsigned8(len),
            access: AccessType::Ro,
        });
        entries
    }
}

impl RpdoMapping {
    pub fn bit_size(&self) -> usize {
        self.items.iter().map(|item| item.length as usize).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn entries() {
        let rpdo = RpdoMapping {
            sequence_number: 1,
            items: vec![
                PdoMappingItem {
                    index: 0x2000,
                    subindex: 1,
                    length: 8,
                },
                PdoMappingItem {
                    index: 0x2010,
                    subindex: 2,
                    length: 16,
                },
            ],
        };
        let left = rpdo.entries();
        let right = vec![
            Entry {
                address: Address {
                    index: 0x1601,
                    subindex: 1,
                },
                value: Value::Unsigned32(0x20000108),
                access: AccessType::Ro,
            },
            Entry {
                address: Address {
                    index: 0x1601,
                    subindex: 2,
                },
                value: Value::Unsigned32(0x20100210),
                access: AccessType::Ro,
            },
            Entry {
                address: Address {
                    index: 0x1601,
                    subindex: 0,
                },
                value: Value::Unsigned8(2),
                access: AccessType::Ro,
            },
        ];

        assert_eq!(left, right)
    }

    #[test]
    fn bit_size() {
        let rpdo = RpdoMapping {
            sequence_number: 1,
            items: vec![
                PdoMappingItem {
                    index: 0x2000,
                    subindex: 1,
                    length: 1,
                },
                PdoMappingItem {
                    index: 0x2010,
                    subindex: 2,
                    length: 8,
                },
                PdoMappingItem {
                    index: 0x2010,
                    subindex: 3,
                    length: 16,
                },
            ],
        };
        assert_eq!(rpdo.bit_size(), 25)
    }
}
