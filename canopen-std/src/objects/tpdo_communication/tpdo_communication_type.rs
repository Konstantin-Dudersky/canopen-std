use crate::types::Value;

/// transmission type
#[derive(Clone, Copy, Debug)]
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

impl From<TpdoTransmissionType> for u8 {
    fn from(value: TpdoTransmissionType) -> Self {
        match value {
            TpdoTransmissionType::SyncronousAcyclic => 0,
            TpdoTransmissionType::Syncronous(value) => value,
            TpdoTransmissionType::EventDrivenManufacturer => 0xFE,
            TpdoTransmissionType::EventDrivenProfile => 0xFF,
            TpdoTransmissionType::Reserved => 0,
        }
    }
}

impl From<TpdoTransmissionType> for Value {
    fn from(value: TpdoTransmissionType) -> Self {
        Self::Unsigned8(value.into())
    }
}
