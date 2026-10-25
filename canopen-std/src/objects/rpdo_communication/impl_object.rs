use std::time::Duration;

use crate::{
    Error,
    types::{AccessType, EntryRecord, Object, ObjectRecord, Value},
};

use super::{
    super::{
        PdoCommunicationCobId,
        common::{decode_cob_id, encode_cob_id},
    },
    RpdoCommunication, RpdoTransmissionType,
};

impl From<RpdoCommunication> for Object {
    fn from(rpdo: RpdoCommunication) -> Self {
        let value_cob_id: Value = match rpdo.cob_id {
            PdoCommunicationCobId::RelativeNodeId => Value::RelativeNodeId(
                rpdo.get_cob_id_offset()
                    .expect("Относительный адрес может быть только для первых 4 конфигураций")
                    as u16,
            ),
            PdoCommunicationCobId::Specific(id) => encode_cob_id(!rpdo.pdo_exist, false, id),
        };

        let value_transmission_type: Value = rpdo.transmission_type.into();

        let value_inhibit_time = rpdo.inhibit_time.as_micros() / 100;
        let value_inhibit_time = Value::Unsigned16(value_inhibit_time as u16);

        let value_event_timer = rpdo.event_timer.as_millis();
        let value_event_timer = Value::Unsigned16(value_event_timer as u16);

        ObjectRecord {
            index: rpdo.index(),
            name: rpdo.name(),
            entries: vec![
                EntryRecord {
                    subindex: RpdoCommunication::SUBINDEX_COB_ID,
                    description: "COB-ID used by RPDO".into(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: value_cob_id,
                },
                EntryRecord {
                    subindex: RpdoCommunication::SUBINDEX_TRANSMISSION_TYPE,
                    description: "transmission type".into(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: value_transmission_type,
                },
                EntryRecord {
                    subindex: RpdoCommunication::SUBINDEX_INHIBIT_TIME,
                    description: "inhibit time".into(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: value_inhibit_time,
                },
                EntryRecord {
                    subindex: RpdoCommunication::SUBINDEX_EVENT_TIMER,
                    description: "event timer".into(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: value_event_timer,
                },
            ],
        }
        .into()
    }
}

impl TryFrom<&Object> for RpdoCommunication {
    type Error = Error;

    fn try_from(obj: &Object) -> Result<Self, Self::Error> {
        let index = obj.index();
        let sequence_number = index - Self::INDEX_BASE;

        // COB-ID used by TPDO
        let subindex = RpdoCommunication::SUBINDEX_COB_ID;
        let entry = obj
            .get_entry(subindex)
            .ok_or(Error::EntryNotFound { index, subindex })?;
        let (pdo_exist, cob_id) = match entry.value {
            Value::Unsigned32(_) => {
                let (pdo_exist, _, cob_id) = decode_cob_id(entry.value)?;
                (pdo_exist, PdoCommunicationCobId::Specific(cob_id))
            }
            Value::RelativeNodeId(_) => (true, PdoCommunicationCobId::RelativeNodeId),
            _ => return Err(Error::TypeMismatch),
        };

        // transmission type
        let subindex = RpdoCommunication::SUBINDEX_TRANSMISSION_TYPE;
        let entry = obj
            .get_entry(subindex)
            .ok_or(Error::EntryNotFound { index, subindex })?;
        let transmission_type: RpdoTransmissionType = entry.value.get_unsigned8()?.into();

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

        Ok(RpdoCommunication {
            sequence_number,
            pdo_exist,
            cob_id,
            transmission_type,
            inhibit_time,
            event_timer,
        })
    }
}
