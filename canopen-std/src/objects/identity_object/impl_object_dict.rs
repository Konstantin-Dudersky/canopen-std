use crate::Error;
use crate::objects::ObjectDictionary;

use super::IdentityObject;

impl ObjectDictionary {
    pub fn read_identity_object(&self) -> Result<IdentityObject, Error> {
        let obj = self.get_object(IdentityObject::INDEX);

        let Some(obj) = obj else {
            let v = IdentityObject::default();
            return Ok(v);
        };

        let v: IdentityObject = obj.try_into()?;
        Ok(v)
    }
}
