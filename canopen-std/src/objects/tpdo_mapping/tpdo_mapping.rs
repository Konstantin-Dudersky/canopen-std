use super::super::common::PdoMappingItem;

/// 0x1A00..=0x1BFF: TPDO mapping
#[derive(Debug, Default)]
pub struct TpdoMapping {
    pub seq_number: u16,
    pub items: Vec<PdoMappingItem>,
}

impl TpdoMapping {
    pub const INDEX_BASE: u16 = 0x1A00;

    pub const fn index(&self) -> u16 {
        Self::INDEX_BASE + self.seq_number
    }

    pub fn name(&self) -> String {
        format!("TPDO mapping parameter #{}", self.seq_number + 1)
    }

    pub fn bit_size(&self) -> usize {
        self.items.iter().map(|item| item.length as usize).sum()
    }
}
