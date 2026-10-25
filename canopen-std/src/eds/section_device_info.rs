// [DeviceInfo]
// VendorName=EK
// VendorNumber={}
// ProductName={}
// ProductNumber={}
// RevisionNumber={}
// OrderCode={}
// BaudRate_10=1
// BaudRate_20=1
// BaudRate_50=1
// BaudRate_125=1
// BaudRate_250=1
// BaudRate_500=1
// BaudRate_800=1
// BaudRate_1000=1
// SimpleBootUpMaster=0
// SimpleBootUpSlave=0
// Granularity=0
// DynamicChannelsSupported=0
// GroupMessaging=0
// NrOfRXPDO={rpdo}
// NrOfTXPDO={tpdo}
// LSS_Supported=0

use std::collections::BTreeMap;

use crate::{Error, objects::ObjectDictionary};

use super::EdsFileSection;

#[derive(Clone, Debug, Default)]
pub struct DeviceInfo {
    pub vendor_name: String,
    pub order_code: String,
}

impl From<&EdsFileSection> for DeviceInfo {
    fn from(str: &BTreeMap<String, String>) -> Self {
        let vendor_name = str.get("vendorname").cloned().unwrap_or_default();
        let order_code = str.get("ordercode").cloned().unwrap_or_default();

        Self {
            vendor_name,
            order_code,
        }
    }
}

impl DeviceInfo {
    pub fn to_eds_str(&self, obj_dict: &ObjectDictionary) -> Result<String, Error> {
        let identity = obj_dict.read_identity_object()?;
        let product_name = obj_dict.read_manufacturer_device_name()?;

        let (rpdo, tpdo) = calc_tpdo_rpdo(obj_dict);

        let eds = format!(
            "
[DeviceInfo]
VendorName={}
VendorNumber={}
ProductName={}
ProductNumber={}
RevisionNumber={}
OrderCode={}
BaudRate_10=1
BaudRate_20=1
BaudRate_50=1
BaudRate_125=1
BaudRate_250=1
BaudRate_500=1
BaudRate_800=1
BaudRate_1000=1
SimpleBootUpMaster=0
SimpleBootUpSlave=0
Granularity=0
DynamicChannelsSupported=0
GroupMessaging=0
NrOfRXPDO={rpdo}
NrOfTXPDO={tpdo}
LSS_Supported=0

",
            self.vendor_name,
            identity.vendor_id,
            product_name.name,
            identity.product_code,
            identity.revision_number,
            self.order_code
        );
        Ok(eds)
    }
}

fn calc_tpdo_rpdo(objects: &ObjectDictionary) -> (u16, u16) {
    let mut rpdo = 0;
    let mut tpdo = 0;
    for obj in objects.iter() {
        match obj.index() {
            0x1400..=0x15FF => rpdo += 1,
            0x1800..=0x19FF => tpdo += 1,
            _ => {}
        }
    }
    (rpdo, tpdo)
}
