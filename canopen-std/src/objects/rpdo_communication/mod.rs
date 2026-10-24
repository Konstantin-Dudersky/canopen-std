mod rpdo_communication;
mod rpdo_transmission_type;

use crate::{objects_old::common, Error};

pub use {rpdo_communication::RpdoCommunication, rpdo_transmission_type::RpdoTransmissionType};

use std::time::Duration;

use embedded_can::Id;

use super::{ObjectDictionary, Value};

const SUBINDEX_TRANSMISSION_TYPE: u8 = 2;

impl ObjectDictionary {
    /// RPDO communication parameter
    #[allow(non_snake_case)]
    pub fn read_rpdo_communication(
        &self,
        sequence_number: u16,
    ) -> Result<RpdoCommunication, Error> {
        let index = sequence_number + 0x1400;

        let cob_id_default =
            RpdoCommunication::default_cob_id(self.node_id.raw(), sequence_number)?;

        let transmission_type_default = RpdoTransmissionType::EventDrivenManufacturer;
        let inhibit_time_default = Duration::new(0, 0);
        let event_timer_default = Duration::new(0, 0);

        // COB-ID used by TPDO
        let entry = self.get_entry(index, 1);
        let Some(entry) = entry else {
            return Ok(RpdoCommunication {
                pdo_exist: false,
                cob_id: cob_id_default,
                transmission_type: transmission_type_default,
                inhibit_time: Duration::new(0, 0),
                event_timer: Duration::new(0, 0),
                sequence_number,
            });
        };
        let (pdo_exist, cob_id) = decode_cob_id(entry.value)?;

        // transmission type
        let entry = self.get_entry(index, SUBINDEX_TRANSMISSION_TYPE);
        let Some(entry) = entry else {
            return Ok(RpdoCommunication {
                pdo_exist,
                cob_id,
                transmission_type: transmission_type_default,
                inhibit_time: Duration::new(0, 0),
                event_timer: Duration::new(0, 0),
                sequence_number,
            });
        };
        let transmission_type: RpdoTransmissionType = entry.value.get_unsigned8()?.into();

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

        Ok(RpdoCommunication {
            pdo_exist,
            cob_id,
            transmission_type,
            inhibit_time,
            event_timer,
            sequence_number,
        })
    }
}

fn decode_cob_id(value: Value) -> Result<(bool, Id), Error> {
    let (bit_31, _, id) = common::decode_cob_id(value)?;

    let pdo_exist = !bit_31;

    Ok((pdo_exist, id))
}
