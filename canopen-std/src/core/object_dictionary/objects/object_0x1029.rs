use super::{Address, Error, ObjectDictionary};

/// Error behavior
#[derive(Debug, PartialEq)]
pub enum Object0x1029ErrorBehavior {
    EnterPreOperational = 0x00,
    NoChange = 0x01,
    Stop = 0x02,
}
impl Object0x1029ErrorBehavior {
    pub fn decode(value: u8) -> Result<Self, Error> {
        let value = match value {
            0x00 => Self::EnterPreOperational,
            0x01 => Self::NoChange,
            0x02 => Self::Stop,
            _ => return Err(Error::ObjectNotFound),
        };
        Ok(value)
    }
}

/// Error behavior
pub struct Object0x1029 {
    /// Communication error behavior
    pub communication_error: Object0x1029ErrorBehavior,

    /// Profile- or manufacturer-specific error
    pub manufacture_errors: Vec<Object0x1029ErrorBehavior>,
}

impl ObjectDictionary {
    pub fn read_object_0x1029(&self) -> Result<Object0x1029, Error> {
        // Определяем количество элементов ---------------------------------------------------------
        let address = Address::new(0x1029, 0);
        let entry = self.entries.get(&address);

        // Возвращаем значение по умолчанию, если элемент не найден
        let Some(highest_subindex) = entry else {
            return Ok(Object0x1029 {
                communication_error: Object0x1029ErrorBehavior::EnterPreOperational,
                manufacture_errors: vec![],
            });
        };
        let highest_subindex = highest_subindex.value.get_unsigned8()?;

        // Определяем поведение при ошибке связи ---------------------------------------------------
        let address = Address::new(0x1029, 1);
        let entry = self.entries.get(&address);

        // Возвращаем значение по умолчанию, если элемент не найден
        let Some(communication_error) = entry else {
            return Ok(Object0x1029 {
                communication_error: Object0x1029ErrorBehavior::EnterPreOperational,
                manufacture_errors: vec![],
            });
        };
        let communication_error = communication_error.value.get_unsigned8()?;
        let communication_error = Object0x1029ErrorBehavior::decode(communication_error)?;

        // Определяем поведение при остальных ошибках ----------------------------------------------
        let mut manufacture_errors = vec![];
        for i in 2..=highest_subindex {
            let address = Address::new(0x1029, i);
            let entry = self.entries.get(&address);
            let Some(entry) = entry else {
                continue;
            };
            let value = entry.value.get_unsigned8()?;
            let value = Object0x1029ErrorBehavior::decode(value)?;
            manufacture_errors.push(value);
        }

        Ok(Object0x1029 {
            communication_error,
            manufacture_errors,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_decode() {
        let left = Object0x1029ErrorBehavior::EnterPreOperational;
        let right = Object0x1029ErrorBehavior::decode(0x00).unwrap();
        assert_eq!(left, right);

        let left = Object0x1029ErrorBehavior::NoChange;
        let right = Object0x1029ErrorBehavior::decode(0x01).unwrap();
        assert_eq!(left, right);

        let left = Object0x1029ErrorBehavior::Stop;
        let right = Object0x1029ErrorBehavior::decode(0x02).unwrap();
        assert_eq!(left, right);
    }
}
