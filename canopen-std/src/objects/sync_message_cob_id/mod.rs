use embedded_can::Id;

use super::{AccessType, DataType, Object, ObjectVar};

/// 0x1005 - COB-ID SYNC
pub struct SyncMessageCobId {
    /// Default: 0x080
    pub cob_id: Id,
    pub device_generate_sync: bool,
}

impl From<SyncMessageCobId> for Object {
    fn from(value: SyncMessageCobId) -> Self {
        ObjectVar {
            index: 0x1005,
            name: "COB-ID SYNC".into(),
            data_type: DataType::Unsigned32,
            access_type: AccessType::Ro,
            pdo_mapping: false,
            default_value: "0x080".into(),
        }
        .into()
    }
}

// fn decode(value: Value) -> Result<SyncMessageCobId, Error> {
//     let (_, device_generate_sync, cob_id) = decode_cob_id(value)?;
//
//     Ok(SyncMessageCobId {
//         cob_id,
//         device_generate_sync,
//     })
// }
//
// fn encode(value: SyncMessageCobId) -> Value {
//     let mut raw: u32 = 0;
//     let bits = raw.view_bits_mut::<Lsb0>();
//
//     match value.cob_id {
//         Id::Standard(id) => {
//             bits.set(29, false);
//             bits[..11].store_le(id.as_raw())
//         }
//         Id::Extended(id) => {
//             bits.set(29, true);
//             bits[..29].store_le(id.as_raw())
//         }
//     }
//     bits.set(30, value.device_generate_sync);
//
//     Value::Unsigned32(raw)
// }
