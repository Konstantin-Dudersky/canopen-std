pub use super::RpdoTransmissionType;

use std::time::Duration;

use embedded_can::{Id, StandardId};
use tracing::error;

use crate::{
    objects_old::common::encode_cob_id,
    types::{AccessType, EntryRecord, Object, ObjectRecord},
    Error,
};

use super::Value;

const SUBINDEX_TRANSMISSION_TYPE: u8 = 2;

/// 0x1400..=0x15FF - RPDO communication parameter
#[derive(Debug)]
pub struct RpdoCommunication {
    pub sequence_number: u16,

    /// 1 = PDO exists
    pub pdo_exist: bool,

    /// COB-ID used by RPDO
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

impl From<RpdoCommunication> for Object {
    fn from(value: RpdoCommunication) -> Self {
        Object::from(ObjectRecord {
            index: 0x1400 + value.sequence_number,
            name: "RPDO communication parameter".into(),
            entries: vec![
                EntryRecord {
                    subindex: 1,
                    description: "COB-ID used by RPDO".into(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: encode_cob_id(false, false, value.cob_id),
                },
                EntryRecord {
                    subindex: SUBINDEX_TRANSMISSION_TYPE,
                    description: "transmission type".into(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: Value::Unsigned8(value.transmission_type.into()),
                },
                EntryRecord {
                    subindex: 3,
                    description: "inhibit time".into(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: Value::Unsigned16(value.inhibit_time.as_micros() as u16 / 100),
                },
                EntryRecord {
                    subindex: 5,
                    description: "event timer".into(),
                    access_type: AccessType::Ro,
                    pdo_mapping: false,
                    value: Value::Unsigned16(value.event_timer.as_millis() as u16),
                },
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use embedded_can::StandardId;
    use pretty_assertions::assert_eq;

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
