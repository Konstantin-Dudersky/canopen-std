use crate::{
    Error,
    types::{AccessType, EntryArray, Object, ObjectArray, Value},
};

use super::{ErrorBehavior, ErrorBehaviorAction};

impl From<ErrorBehavior> for Object {
    fn from(value: ErrorBehavior) -> Self {
        let mut entries = vec![];

        let entry_communication_error = EntryArray {
            subindex: 1,
            description: "Communication error".into(),
            access_type: AccessType::Rw,
            pdo_mapping: false,
            value: Value::Unsigned8(value.communication_error.into()),
        };
        entries.push(entry_communication_error);

        for (subindex, action) in value.manufacture_errors.iter().enumerate() {
            let entry = EntryArray {
                subindex: (subindex + 2) as u8, // пропускаем communication error
                description: format!("Manufacture error #{}", subindex),
                access_type: AccessType::Rw,
                pdo_mapping: false,
                value: Value::Unsigned8(action.into()),
            };
            entries.push(entry);
        }

        ObjectArray {
            index: ErrorBehavior::INDEX,
            name: ErrorBehavior::NAME.to_string(),
            entries,
        }
        .into()
    }
}

impl TryFrom<&Object> for ErrorBehavior {
    type Error = Error;

    fn try_from(value: &Object) -> Result<Self, Self::Error> {
        let entries = value.get_all_entries_without_highest_subindex();

        let entry_communication_error =
            entries
                .iter()
                .find(|e| e.subindex == 1)
                .ok_or(Error::EntryNotFound {
                    index: Self::INDEX,
                    subindex: 1,
                })?;
        let communication_error: ErrorBehaviorAction = entry_communication_error
            .value
            .get_unsigned8()?
            .try_into()?;

        let manufacture_errors: Vec<ErrorBehaviorAction> = entries
            .iter()
            .filter(|e| e.subindex > 1)
            .map(|e| e.value.get_unsigned8()?.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            communication_error,
            manufacture_errors,
        })
    }
}
