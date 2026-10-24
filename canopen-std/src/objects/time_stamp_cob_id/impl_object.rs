use crate::{
    Error,
    objects_old::common::{decode_cob_id, encode_cob_id},
    types::{AccessType, Object, ObjectVar},
};

use super::TimeStampCobId;

impl From<TimeStampCobId> for Object {
    fn from(value: TimeStampCobId) -> Self {
        let value = encode_cob_id(
            value.device_consume_time,
            value.device_produce_time,
            value.cob_id,
        );

        ObjectVar {
            index: TimeStampCobId::INDEX,
            name: TimeStampCobId::NAME.into(),
            access_type: AccessType::Rw,
            pdo_mapping: false,
            value,
        }
        .into()
    }
}

impl TryFrom<&Object> for TimeStampCobId {
    type Error = Error;

    fn try_from(obj: &Object) -> Result<Self, Self::Error> {
        let entry = obj.get_entry(0).ok_or(Error::EntryNotFound {
            index: Self::INDEX,
            subindex: 0,
        })?;

        let (device_consume_time, device_produce_time, cob_id) = decode_cob_id(entry.value)?;

        Ok(TimeStampCobId {
            device_consume_time,
            device_produce_time,
            cob_id,
        })
    }
}

impl TryFrom<Object> for TimeStampCobId {
    type Error = Error;

    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        TimeStampCobId::try_from(&obj)
    }
}
