use crate::{
    Error,
    types::{AccessType, Object, ObjectVar, Value},
};

use super::SyncCounterOverflow;

impl From<SyncCounterOverflow> for Object {
    fn from(value: SyncCounterOverflow) -> Self {
        let mut counter = value.counter;

        let counter_correct = counter == 0 || (2..=240).contains(&counter);
        if !counter_correct {
            counter = 0;
        }

        ObjectVar {
            index: SyncCounterOverflow::INDEX,
            name: SyncCounterOverflow::NAME.to_string(),
            access_type: AccessType::Rw,
            pdo_mapping: false,
            value: Value::Unsigned8(counter),
        }
        .into()
    }
}

impl TryFrom<&Object> for SyncCounterOverflow {
    type Error = Error;

    fn try_from(value: &Object) -> Result<Self, Self::Error> {
        let entry = value.get_entry(0).ok_or(Error::EntryNotFound {
            index: Self::INDEX,
            subindex: 0,
        })?;

        let counter = entry.value.get_unsigned8()?;
        let counter = match counter {
            0 | 2..=240 => counter,
            _ => {
                return Err(Error::WrongDatatype {
                    description: "Invalid value 0x1019",
                    value: entry.value,
                });
            }
        };

        Ok(SyncCounterOverflow { counter })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_read_0x1019() {
        let obj: Object = ObjectVar {
            index: 0x1019,
            name: "SyncCounterOverflow".to_string(),
            access_type: AccessType::Rw,
            pdo_mapping: false,
            value: Value::Unsigned8(0),
        }
        .into();
        let sync = SyncCounterOverflow::try_from(&obj).unwrap();
        assert_eq!(0, sync.counter);

        let obj: Object = ObjectVar {
            index: 0x1019,
            name: "SyncCounterOverflow".to_string(),
            access_type: AccessType::Rw,
            pdo_mapping: false,
            value: Value::Unsigned8(1),
        }
        .into();
        let sync = SyncCounterOverflow::try_from(&obj);
        assert_eq!(true, sync.is_err());

        let obj: Object = ObjectVar {
            index: 0x1019,
            name: "SyncCounterOverflow".to_string(),
            access_type: AccessType::Rw,
            pdo_mapping: false,
            value: Value::Unsigned8(2),
        }
        .into();
        let sync = SyncCounterOverflow::try_from(&obj).unwrap();
        assert_eq!(2, sync.counter);
    }
}
