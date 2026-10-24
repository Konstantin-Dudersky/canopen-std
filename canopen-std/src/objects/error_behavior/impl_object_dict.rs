use crate::{Error, objects::ObjectDictionary};

use super::ErrorBehavior;

impl ObjectDictionary {
    pub fn read_error_behavior(&self) -> Result<ErrorBehavior, Error> {
        let obj = self.get_object(ErrorBehavior::INDEX);
        let Some(obj) = obj else {
            return Ok(ErrorBehavior::default());
        };

        let v: ErrorBehavior = obj.try_into()?;

        Ok(v)
    }
}
