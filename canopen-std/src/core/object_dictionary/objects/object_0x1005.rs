use crate::core::sync::SYNC_COB_ID;
use bitvec::{field::BitField, order::Lsb0, view::BitView};
use embedded_can::{ExtendedId, Id, StandardId};

use super::{Address, Error, ObjectDictionary};

/// COB-ID SYNC
pub struct Object0x1005 {
    /// Default: 0x080
    pub cob_id: Id,
    pub device_generate_sync: bool,
}

impl ObjectDictionary {
    pub fn read_0x1005(&self) -> Result<Object0x1005, Error> {
        let address = Address::new(0x1005, 0);
        let entry = self.entries.get(&address);
        let Some(entry) = entry else {
            return Ok(Object0x1005 {
                cob_id: SYNC_COB_ID,
                device_generate_sync: false,
            });
        };

        let raw = entry.value.get_unsigned32()?;
        let obj = decode(raw)?;

        Ok(obj)
    }
}

fn decode(raw: u32) -> Result<Object0x1005, Error> {
    let bits = raw.view_bits::<Lsb0>();
    let device_generate_sync = bits[30];
    let frame = bits[29];
    let cob_id = match frame {
        true => {
            let id_raw: u32 = bits[0..29].load_le();
            let id = ExtendedId::new(id_raw).ok_or(Error::InvalidCobId)?;
            Id::Extended(id)
        }
        false => {
            let id_raw: u16 = bits[0..11].load_le();
            let id = StandardId::new(id_raw).ok_or(Error::InvalidCobId)?;
            Id::Standard(id)
        }
    };

    Ok(Object0x1005 {
        cob_id,
        device_generate_sync,
    })
}
