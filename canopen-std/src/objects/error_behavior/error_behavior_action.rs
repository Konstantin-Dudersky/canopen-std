use crate::Error;

/// Error behavior
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ErrorBehaviorAction {
    #[default]
    EnterPreOperational = 0x00,
    NoChange = 0x01,
    Stop = 0x02,
}

impl TryFrom<u8> for ErrorBehaviorAction {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::EnterPreOperational),
            0x01 => Ok(Self::NoChange),
            0x02 => Ok(Self::Stop),
            _ => Err(Error::ObjectNotFound),
        }
    }
}

impl From<ErrorBehaviorAction> for u8 {
    fn from(value: ErrorBehaviorAction) -> Self {
        Self::from(&value)
    }
}

impl From<&ErrorBehaviorAction> for u8 {
    fn from(value: &ErrorBehaviorAction) -> Self {
        match value {
            ErrorBehaviorAction::EnterPreOperational => 0x00,
            ErrorBehaviorAction::NoChange => 0x01,
            ErrorBehaviorAction::Stop => 0x02,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_decode() {
        let left = ErrorBehaviorAction::EnterPreOperational;
        let right: ErrorBehaviorAction = 0x00.try_into().unwrap();
        assert_eq!(left, right);

        let left = ErrorBehaviorAction::NoChange;
        let right: ErrorBehaviorAction = 0x01.try_into().unwrap();
        assert_eq!(left, right);

        let left = ErrorBehaviorAction::Stop;
        let right: ErrorBehaviorAction = 0x02.try_into().unwrap();
        assert_eq!(left, right);
    }
}
