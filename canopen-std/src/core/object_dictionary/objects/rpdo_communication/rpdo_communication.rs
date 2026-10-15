pub use super::RpdoTransmissionType;

use std::time::Duration;

use embedded_can::{Id, StandardId};
use tracing::error;

use crate::{AccessType, Entry, Error};

use super::{
    super::common::{encode_cob_id, ObjectTrait},
    Address, Value,
};

const SUBINDEX_TRANSMISSION_TYPE: u8 = 2;

/// 0x1400..=0x15FF - RPDO communication parameter
#[derive(Debug)]
pub struct RpdoCommunication {
    pub sequence_number: u16,

    /// 1 = PDO exists
    pub pdo_exist: bool,

    /// COB-ID used by TPDO
    ///
    /// Default values:
    /// - 0x1400 - 0x200 + Node-ID
    /// - 0x1401 - 0x300 + Node-ID
    /// - 0x1402 - 0x400 + Node-ID
    /// - 0x1403 - 0x500 + Node-ID
    pub cob_id: Id,

    /// transmission type
    pub transmission_type: RpdoTransmissionType,

    /// inhibit time
    pub inhibit_time: Duration,

    /// event timer
    pub event_timer: Duration,
}

impl RpdoCommunication {
    pub fn default_cob_id(node_id: u8, sequence_number: u16) -> Result<Id, Error> {
        let cob_id_base = match sequence_number {
            0 => 0x200,
            1 => 0x300,
            2 => 0x400,
            3 => 0x500,
            _ => {
                error!("Max sequence_number for RPDO currently 3");
                return Err(Error::InvalidCobId);
            }
        };
        let cob_id = StandardId::new(cob_id_base + node_id as u16).ok_or(Error::InvalidCobId)?;
        Ok(cob_id.into())
    }
}

impl ObjectTrait for RpdoCommunication {
    fn entries(self) -> Vec<Entry> {
        let highest_index = Entry {
            address: Address {
                index: 0x1400 + self.sequence_number,
                subindex: 0,
            },
            value: Value::Unsigned8(5),
            access: AccessType::Ro,
        };

        let cob_id = Entry {
            address: Address {
                index: 0x1400 + self.sequence_number,
                subindex: 1,
            },
            value: encode_cob_id(false, false, self.cob_id),
            access: AccessType::Ro,
        };

        let transmission_type = Entry {
            address: Address {
                index: 0x1400 + self.sequence_number,
                subindex: SUBINDEX_TRANSMISSION_TYPE,
            },
            value: Value::Unsigned8(self.transmission_type.into()),
            access: AccessType::Ro,
        };

        let inhibit_time = Entry {
            address: Address {
                index: 0x1400 + self.sequence_number,
                subindex: 3,
            },
            value: Value::Unsigned16(self.inhibit_time.as_micros() as u16 / 100),
            access: AccessType::Ro,
        };

        let event_timer = Entry {
            address: Address {
                index: 0x1400 + self.sequence_number,
                subindex: 5,
            },
            value: Value::Unsigned16(self.event_timer.as_millis() as u16),
            access: AccessType::Ro,
        };

        vec![
            highest_index,
            cob_id,
            transmission_type,
            inhibit_time,
            event_timer,
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::{objects::common::ObjectTrait, AccessType, Entry};

    use super::*;

    use embedded_can::StandardId;
    use pretty_assertions::assert_eq;

    #[test]
    fn entries() {
        let params = RpdoCommunication {
            pdo_exist: true,
            cob_id: Id::Standard(StandardId::new(0x201).unwrap()),
            transmission_type: RpdoTransmissionType::EventDrivenManufacturer,
            inhibit_time: Duration::from_millis(10),
            event_timer: Duration::from_millis(1000),
            sequence_number: 1,
        };

        let left = params.entries();

        let right = vec![
            Entry {
                address: Address::new(0x1401, 0),
                value: Value::Unsigned8(5),
                access: AccessType::Ro,
            },
            Entry {
                address: Address::new(0x1401, 1),
                value: Value::Unsigned32(0x201),
                access: AccessType::Ro,
            },
            Entry {
                address: Address::new(0x1401, SUBINDEX_TRANSMISSION_TYPE),
                value: Value::Unsigned8(0xFE),
                access: AccessType::Ro,
            },
            Entry {
                address: Address::new(0x1401, 3),
                value: Value::Unsigned16(100),
                access: AccessType::Ro,
            },
            Entry {
                address: Address::new(0x1401, 5),
                value: Value::Unsigned16(1000),
                access: AccessType::Ro,
            },
        ];

        assert_eq!(left, right);
    }

    #[test]
    fn default_cob_id() {
        let cob_id = RpdoCommunication::default_cob_id(0x17, 1);

        assert!(cob_id.is_ok());
        assert_eq!(
            cob_id.unwrap(),
            Id::Standard(StandardId::new(0x317).unwrap())
        );
    }
}
