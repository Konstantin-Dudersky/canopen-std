use crate::eds::EdsError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectType {
    Var,
    Array,
    Record,
}

impl TryFrom<&str> for ObjectType {
    type Error = EdsError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let value = if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
            u8::from_str_radix(hex, 16)
        } else {
            s.parse::<u8>()
        };
        let value = value.map_err(|_| EdsError::UnknownObjectType(s.into()))?;

        match value {
            0x07 => Ok(Self::Var),
            0x08 => Ok(Self::Array),
            0x09 => Ok(Self::Record),
            _ => Err(EdsError::UnknownObjectType(s.into())),
        }
    }
}

impl From<ObjectType> for u8 {
    fn from(value: ObjectType) -> Self {
        match value {
            ObjectType::Var => 0x07,
            ObjectType::Array => 0x08,
            ObjectType::Record => 0x09,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from() -> anyhow::Result<()> {
        let left = ObjectType::try_from("0x07")?;
        let right = ObjectType::Var;
        assert_eq!(left, right);

        let left = ObjectType::try_from("7")?;
        let right = ObjectType::Var;
        assert_eq!(left, right);

        let left = ObjectType::try_from("0x08")?;
        let right = ObjectType::Array;
        assert_eq!(left, right);

        let left = ObjectType::try_from("0x09")?;
        let right = ObjectType::Record;
        assert_eq!(left, right);

        Ok(())
    }
}
