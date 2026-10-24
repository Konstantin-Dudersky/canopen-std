use bitvec::{field::BitField, order::Lsb0, view::BitView};
use embedded_can::Id;

use crate::{Error, core::sync::SYNC_COB_ID, objects_old::common::decode_cob_id};

use super::{AccessType, Object, ObjectDictionary, ObjectVar, Value};

/// 0x1005: COB-ID SYNC
pub struct SyncMessageCobId {
    /// Default: 0x080
    pub cob_id: Id,
    pub device_generate_sync: bool,
}

impl ObjectDictionary {
    pub fn read_sync_message_cob_id(&self) -> Result<SyncMessageCobId, Error> {
        let entry = self.get_entry(0x1005, 0);
        let Some(entry) = entry else {
            return Ok(SyncMessageCobId {
                cob_id: SYNC_COB_ID,
                device_generate_sync: false,
            });
        };

        let obj = decode(entry.value)?;

        Ok(obj)
    }
}

impl From<SyncMessageCobId> for Object {
    fn from(value: SyncMessageCobId) -> Self {
        ObjectVar {
            index: 0x1005,
            name: "COB-ID SYNC".into(),
            access_type: AccessType::Ro,
            pdo_mapping: false,
            value: encode(value),
        }
        .into()
    }
}

fn decode(value: Value) -> Result<SyncMessageCobId, Error> {
    let (_, device_generate_sync, cob_id) = decode_cob_id(value)?;

    Ok(SyncMessageCobId {
        cob_id,
        device_generate_sync,
    })
}

fn encode(value: SyncMessageCobId) -> Value {
    let mut raw: u32 = 0;
    let bits = raw.view_bits_mut::<Lsb0>();

    match value.cob_id {
        Id::Standard(id) => {
            bits.set(29, false);
            bits[..11].store_le(id.as_raw())
        }
        Id::Extended(id) => {
            bits.set(29, true);
            bits[..29].store_le(id.as_raw())
        }
    }
    bits.set(30, value.device_generate_sync);

    Value::Unsigned32(raw)
}
