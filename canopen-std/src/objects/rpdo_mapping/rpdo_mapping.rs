use super::super::common::PdoMappingItem;

/// 0x1600..=0x17FF: RPDO mapping parameter
#[derive(Debug, Default)]
pub struct RpdoMapping {
    pub seq_number: u16,
    pub items: Vec<PdoMappingItem>,
}

impl RpdoMapping {
    pub const INDEX_BASE: u16 = 0x1600;

    pub const fn index(&self) -> u16 {
        Self::INDEX_BASE + self.seq_number
    }

    pub fn name(&self) -> String {
        format!("RPDO mapping parameter #{}", self.seq_number + 1)
    }

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
            seq_number: 1,
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
