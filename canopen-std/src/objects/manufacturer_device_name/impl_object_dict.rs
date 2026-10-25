use crate::Error;
use crate::objects::ObjectDictionary;

use super::ManufacturerDeviceName;

impl ObjectDictionary {
    pub fn read_manufacturer_device_name(&self) -> Result<ManufacturerDeviceName, Error> {
        let obj = self.get_object(ManufacturerDeviceName::INDEX);

        let Some(obj) = obj else {
            let v = ManufacturerDeviceName::default();
            return Ok(v);
        };

        let v: ManufacturerDeviceName = obj.try_into()?;
        Ok(v)
    }
}
