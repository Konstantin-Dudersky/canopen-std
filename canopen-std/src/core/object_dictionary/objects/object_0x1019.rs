use super::{Address, Error, ObjectDictionary, Value};

/// Synchronous counter overflow value
#[derive(Debug)]
pub struct Object0x1019 {
    /// Synchronous counter overflow value
    pub counter: u8,
}

impl ObjectDictionary {
    pub fn read_0x1019(&self) -> Result<Object0x1019, Error> {
        let address = Address::new(0x1019, 0);
        let entry = self.entries.get(&address);
        let Some(entry) = entry else {
            return Ok(Object0x1019 { counter: 0 });
        };

        let value = decode(entry.value)?;

        Ok(Object0x1019 { counter: value })
    }
}

fn decode(value: Value) -> Result<u8, Error> {
    let raw = value.get_unsigned8()?;

    match raw {
        0 | 2..=240 => Ok(raw),
        _ => Err(Error::WrongDatatype {
            description: "Invalid value 0x1019",
            value,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_read_0x1019() {
        assert_eq!(0, decode(Value::Unsigned8(0)).unwrap());
        assert_eq!(true, decode(Value::Unsigned8(1)).is_err());
        assert_eq!(2, decode(Value::Unsigned8(2)).unwrap());
    }
}
