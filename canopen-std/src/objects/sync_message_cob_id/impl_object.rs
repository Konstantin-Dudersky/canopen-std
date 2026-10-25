use crate::{
    Error,
    types::{AccessType, Object, ObjectVar},
};

use super::{
    super::common::{decode_cob_id, encode_cob_id},
    SyncMessageCobId,
};

impl From<SyncMessageCobId> for Object {
    fn from(value: SyncMessageCobId) -> Self {
        let value = encode_cob_id(false, value.device_generate_sync, value.cob_id);

        ObjectVar {
            index: SyncMessageCobId::INDEX,
            name: SyncMessageCobId::NAME.into(),
            access_type: AccessType::Ro,
            pdo_mapping: false,
            value,
        }
        .into()
    }
}

impl TryFrom<&Object> for SyncMessageCobId {
    type Error = Error;

    fn try_from(obj: &Object) -> Result<Self, Self::Error> {
        let entry = obj.get_entry(0).ok_or(Error::EntryNotFound {
            index: Self::INDEX,
            subindex: 0,
        })?;

        let (_, device_generate_sync, cob_id) = decode_cob_id(entry.value)?;

        Ok(Self {
            cob_id,
            device_generate_sync,
        })
    }
}
