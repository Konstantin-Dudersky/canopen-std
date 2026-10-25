use std::time::Duration;

use crate::{
    Error,
    objects_old::common::{decode_cob_id, encode_cob_id},
    types::{AccessType, EntryRecord, Object, ObjectRecord, Value},
};

use super::{TpdoCommunication, TpdoCommunicationCobId, TpdoTransmissionType};

impl From<TpdoCommunication> for Object {
    fn from(tpdo: TpdoCommunication) -> Self {
        let value_cob_id: Value = match tpdo.cob_id {
            super::TpdoCommunicationCobId::RelativeNodeId => {
                Value::RelativeNodeId(tpdo.sequence_number)
            }
            super::TpdoCommunicationCobId::Specific(id) => {
                encode_cob_id(!tpdo.pdo_exist, false, id)
            }
        };

        let value_transmission_type: Value = tpdo.transmission_type.into();

        let value_inhibit_time = tpdo.inhibit_time.as_micros() / 100;
        let value_inhibit_time = Value::Unsigned16(value_inhibit_time as u16);

        let value_event_timer = tpdo.event_timer.as_millis();
        let value_event_timer = Value::Unsigned16(value_event_timer as u16);

        let value_sync_start_value = tpdo.sync_start_value;
        let value_sync_start_value = Value::Unsigned8(value_sync_start_value);

        ObjectRecord {
            index: tpdo.index(),
            name: tpdo.name(),
            entries: vec![
                EntryRecord {
                    subindex: TpdoCommunication::SUBINDEX_COB_ID,
                    description: "COB-ID used by TPDO".to_string(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: value_cob_id,
                },
                EntryRecord {
                    subindex: TpdoCommunication::SUBINDEX_TRANSMISSION_TYPE,
                    description: "transmission type".into(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: value_transmission_type,
                },
                EntryRecord {
                    subindex: TpdoCommunication::SUBINDEX_INHIBIT_TIME,
                    description: "inhibit time".into(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: value_inhibit_time,
                },
                EntryRecord {
                    subindex: TpdoCommunication::SUBINDEX_EVENT_TIMER,
                    description: "event timer".into(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: value_event_timer,
                },
                EntryRecord {
                    subindex: TpdoCommunication::SUBINDEX_SYNC_START_VALUE,
                    description: "SYNC start value".into(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: value_sync_start_value,
                },
            ],
        }
        .into()
    }
}

impl TryFrom<&Object> for TpdoCommunication {
    type Error = Error;

    fn try_from(obj: &Object) -> Result<Self, Self::Error> {
        let index = obj.index();
        let sequence_number = index - 0x1800;

        // COB-ID used by TPDO
        let subindex = TpdoCommunication::SUBINDEX_COB_ID;
        let entry = obj
            .get_entry(subindex)
            .ok_or(Error::EntryNotFound { index, subindex })?;
        let (pdo_exist, cob_id) = match entry.value {
            Value::Unsigned32(_) => {
                let (pdo_exist, _, cob_id) = decode_cob_id(entry.value)?;
                (pdo_exist, TpdoCommunicationCobId::Specific(cob_id))
            }
            Value::RelativeNodeId(_) => (true, TpdoCommunicationCobId::RelativeNodeId),
            _ => return Err(Error::TypeMismatch),
        };

        // transmission type
        let subindex = TpdoCommunication::SUBINDEX_TRANSMISSION_TYPE;
        let entry = obj
            .get_entry(subindex)
            .ok_or(Error::EntryNotFound { index, subindex })?;
        let transmission_type: TpdoTransmissionType = entry.value.get_unsigned8()?.into();

        // inhibit time
        let entry = obj.get_entry(Self::SUBINDEX_INHIBIT_TIME);
        let inhibit_time = match entry {
            Some(v) => {
                let inhibit_time_us = v.value.get_unsigned16()? as u64 * 100;
                Duration::from_micros(inhibit_time_us)
            }
            None => Duration::new(0, 0),
        };

        // event timer
        let entry = obj.get_entry(Self::SUBINDEX_EVENT_TIMER);
        let event_timer = match entry {
            Some(v) => {
                let event_timer_ms = v.value.get_unsigned16()? as u64;
                Duration::from_millis(event_timer_ms)
            }
            None => Duration::new(0, 0),
        };

        // SYNC start value
        let entry = obj.get_entry(Self::SUBINDEX_SYNC_START_VALUE);
        let sync_start_value = match entry {
            Some(v) => v.value.get_unsigned8()?,
            None => 0,
        };

        Ok(TpdoCommunication {
            sequence_number,
            pdo_exist,
            cob_id,
            transmission_type,
            inhibit_time,
            event_timer,
            sync_start_value,
        })
    }
}
