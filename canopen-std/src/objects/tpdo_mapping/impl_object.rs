use crate::Error;
use crate::types::{EntryRecord, Object, ObjectRecord};

use super::super::PdoMappingItem;
use super::TpdoMapping;

impl From<TpdoMapping> for Object {
    fn from(value: TpdoMapping) -> Self {
        let index = value.index();
        let name = value.name();

        let entries: Vec<EntryRecord> =
            PdoMappingItem::convert_items_to_entry_records(&value.items);

        ObjectRecord {
            index,
            name,
            entries,
        }
        .into()
    }
}

impl TryFrom<&Object> for TpdoMapping {
    type Error = Error;

    fn try_from(obj: &Object) -> Result<Self, Self::Error> {
        let index = obj.index();
        let sequence_number = index - Self::INDEX_BASE;

        let entries = obj.get_all_entries_without_highest_subindex();
        let items = PdoMappingItem::convert_entries_to_items(&entries)?;

        Ok(TpdoMapping {
            seq_number: sequence_number,
            items,
        })
    }
}
