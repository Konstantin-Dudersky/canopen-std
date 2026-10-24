use crate::{
    objects_old::PdoMappingItem,
    types::{AccessType, EntryRecord, Object, ObjectRecord},
};

/// 0x1600..=0x17FF - RPDO mapping parameter
#[derive(Debug)]
pub struct RpdoMapping {
    pub sequence_number: u16,
    pub items: Vec<PdoMappingItem>,
}

impl From<RpdoMapping> for Object {
    fn from(value: RpdoMapping) -> Self {
        let index = 0x1600 + value.sequence_number;

        let entries: Vec<EntryRecord> = value
            .items
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let subindex = i as u8 + 1;
                EntryRecord {
                    subindex,
                    description: "Mapping item".to_string(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: item.into(),
                }
            })
            .collect::<Vec<_>>();

        ObjectRecord {
            index,
            name: "RPDO Mapping".to_string(),
            entries,
        }
        .into()
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
