use crate::{
    Error,
    types::{AccessType, EntryArray, Object, ObjectArray, Value},
};

use super::{ConsumerHeartbeatTime, ConsumerHeartbeatTimeItem};

impl From<ConsumerHeartbeatTime> for Object {
    fn from(value: ConsumerHeartbeatTime) -> Self {
        let mut entries = vec![];

        for (subindex, item) in value.items.iter().enumerate() {
            let value: Value = item.into();
            let description = format!("{} #{}", ConsumerHeartbeatTime::NAME, subindex + 1);

            let entry = EntryArray {
                subindex: subindex as u8,
                description,
                access_type: AccessType::Rw,
                pdo_mapping: false,
                value,
            };
            entries.push(entry);
        }

        ObjectArray {
            index: ConsumerHeartbeatTime::INDEX,
            name: ConsumerHeartbeatTime::NAME.to_string(),
            entries,
        }
        .into()
    }
}

impl TryFrom<&Object> for ConsumerHeartbeatTime {
    type Error = Error;

    fn try_from(obj: &Object) -> Result<Self, Self::Error> {
        let items = obj
            .get_all_entries_without_highest_subindex()
            .iter()
            .map(|e| {
                let item: Result<ConsumerHeartbeatTimeItem, _> = e.value.try_into();
                item
            })
            .collect::<Result<Vec<ConsumerHeartbeatTimeItem>, _>>()?;

        Ok(ConsumerHeartbeatTime { items })
    }
}
