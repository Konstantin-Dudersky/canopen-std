use std::fmt::Display;

/// transmission type
#[derive(Debug)]
pub enum RpdoTransmissionType {
    Syncronous,
    EventDrivenManufacturer,
    EventDrivenProfile,
    Reserved,
}

impl From<u8> for RpdoTransmissionType {
    fn from(value: u8) -> Self {
        match value {
            0..=0xF0 => RpdoTransmissionType::Syncronous,
            0xFE => RpdoTransmissionType::EventDrivenManufacturer,
            0xFF => RpdoTransmissionType::EventDrivenProfile,
            _ => RpdoTransmissionType::Reserved,
        }
    }
}

impl From<RpdoTransmissionType> for u8 {
    fn from(value: RpdoTransmissionType) -> Self {
        match value {
            RpdoTransmissionType::Syncronous => 0,
            RpdoTransmissionType::EventDrivenManufacturer => 0xFE,
            RpdoTransmissionType::EventDrivenProfile => 0xFF,
            RpdoTransmissionType::Reserved => 0xF1,
        }
    }
}

impl Display for RpdoTransmissionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RpdoTransmissionType::Syncronous => "syncronous",
            RpdoTransmissionType::EventDrivenManufacturer => "event-driven (manufacturer-specific)",
            RpdoTransmissionType::EventDrivenProfile => {
                "event-driven (device profile and application profile specific)"
            }
            RpdoTransmissionType::Reserved => "reserved",
        };

        write!(f, "{}", str)
    }
}
