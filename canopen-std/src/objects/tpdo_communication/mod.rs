use std::time::Duration;

use embedded_can::{Id, StandardId};

use crate::{objects_old::common, Error};

use super::{ObjectDictionary, Value};

/// transmission type
#[derive(Debug)]
pub enum TpdoTransmissionType {
    SyncronousAcyclic,
    Syncronous(u8),
    EventDrivenManufacturer,
    EventDrivenProfile,
    Reserved,
}

impl From<u8> for TpdoTransmissionType {
    fn from(value: u8) -> Self {
        match value {
            0 => TpdoTransmissionType::SyncronousAcyclic,
            1..=0xF0 => TpdoTransmissionType::Syncronous(value),
            0xFE => TpdoTransmissionType::EventDrivenManufacturer,
            0xFF => TpdoTransmissionType::EventDrivenProfile,
            _ => TpdoTransmissionType::Reserved,
        }
    }
}

/// TPDO communication parameter
#[derive(Debug)]
pub struct TpdoCommunication {
    /// 1 = PDO exists
    pub pdo_exist: bool,

    /// COB-ID used by TPDO
    pub cob_id: Id,

    /// transmission type
    pub transmission_type: TpdoTransmissionType,

    /// inhibit time
    pub inhibit_time: Duration,

    /// event timer
    pub event_timer: Duration,

    /// SYNC start value
    pub sync_start_value: u8,
}

impl ObjectDictionary {
    pub fn read_tpdo_communication(&self, tpdo_index: u16) -> Result<TpdoCommunication, Error> {
        let index = 0x1800 + tpdo_index;

        let cob_id_base = 0x180;
        let cob_id_default: Id = StandardId::new(cob_id_base + self.node_id.raw() as u16)
            .unwrap()
            .into();
        let transmission_type_default = TpdoTransmissionType::SyncronousAcyclic;
        let inhibit_time_default = Duration::new(0, 0);
        let event_timer_default = Duration::new(0, 0);
        let sync_start_value_default = 0;

        // COB-ID used by TPDO
        let entry = self.get_entry(index, 1);
        let Some(entry) = entry else {
            return Ok(TpdoCommunication {
                pdo_exist: false,
                cob_id: cob_id_default,
                transmission_type: transmission_type_default,
                inhibit_time: Duration::new(0, 0),
                event_timer: Duration::new(0, 0),
                sync_start_value: 0,
            });
        };
        let (pdo_exist, cob_id) = decode_cob_id(entry.value)?;

        // transmission type
        let entry = self.get_entry(index, 2);
        let Some(entry) = entry else {
            return Ok(TpdoCommunication {
                pdo_exist,
                cob_id,
                transmission_type: transmission_type_default,
                inhibit_time: Duration::new(0, 0),
                event_timer: Duration::new(0, 0),
                sync_start_value: 0,
            });
        };
        let transmission_type: TpdoTransmissionType = entry.value.get_unsigned8()?.into();

        // inhibit time
        let entry = self.get_entry(index, 3);
        let inhibit_time = match entry {
            Some(v) => {
                let inhibit_time_us = v.value.get_unsigned16()? as u64 * 100;
                Duration::from_micros(inhibit_time_us)
            }
            None => inhibit_time_default,
        };

        // event timer
        let entry = self.get_entry(index, 5);
        let event_timer = match entry {
            Some(v) => {
                let event_timer_ms = v.value.get_unsigned16()? as u64;
                Duration::from_millis(event_timer_ms)
            }
            None => event_timer_default,
        };

        // SYNC start value
        let entry = self.get_entry(index, 6);
        let sync_start_value = match entry {
            Some(v) => v.value.get_unsigned8()?,
            None => sync_start_value_default,
        };

        Ok(TpdoCommunication {
            pdo_exist,
            cob_id,
            transmission_type,
            inhibit_time,
            event_timer,
            sync_start_value,
        })
    }
}

fn decode_cob_id(value: Value) -> Result<(bool, Id), Error> {
    let (bit_31, _, id) = common::decode_cob_id(value)?;

    let pdo_exist = !bit_31;

    Ok((pdo_exist, id))
}
