/// A CANopen basic data type.
///
/// The discriminant is the CiA 301 data type index (the object index under
/// which the type is described, e.g. `UNSIGNED32` is `0x0007`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DataType {
    /// `BOOLEAN` (0x0001), 1 byte.
    Boolean = 0x01,
    /// `INTEGER8` (0x0002), 1 byte.
    Integer8 = 0x02,
    /// `INTEGER16` (0x0003), 2 bytes.
    Integer16 = 0x03,
    /// `INTEGER32` (0x0004), 4 bytes.
    Integer32 = 0x04,
    /// `UNSIGNED8` (0x0005), 1 byte.
    Unsigned8 = 0x05,
    /// `UNSIGNED16` (0x0006), 2 bytes.
    Unsigned16 = 0x06,
    /// `UNSIGNED32` (0x0007), 4 bytes.
    Unsigned32 = 0x07,
    /// `REAL32` (0x0008), 4 bytes.
    Real32 = 0x08,
    /// `REAL64` (0x0011), 8 bytes.
    Real64 = 0x11,
    /// `INTEGER64` (0x0015), 8 bytes.
    Integer64 = 0x15,
    /// `UNSIGNED64` (0x001B), 8 bytes.
    Unsigned64 = 0x1B,
    /// `VISIBLE_STRING` (0x0009), variable length (ASCII/UTF-8 text).
    VisibleString = 0x09,
    /// `OCTET_STRING` (0x000A), variable length (arbitrary bytes).
    OctetString = 0x0A,
    /// `DOMAIN` (0x000F), variable length (arbitrary bytes).
    Domain = 0x0F,
    /// `PDO_COMMUNICATION_PARAMETER` (0x0020)
    PdoCommunicationParameter = 0x0020,
    /// `PDO_MAPPING` (0x0021)
    PdoMapping = 0x0021,
    /// `SDO_PARAMETER` (0x0022)
    SdoParameter = 0x0022,
    /// `IDENTITY` (0x0023)
    Identity = 0x0023,
}

impl DataType {
    /// The CiA 301 data type index (e.g. `0x0007` for `UNSIGNED32`).
    pub const fn index(self) -> u16 {
        self as u16
    }

    /// The basic data type for a CiA 301 data type index, or `None` for an
    /// index this crate does not model.
    pub const fn from_index(index: u16) -> Option<Self> {
        Some(match index {
            0x01 => DataType::Boolean,
            0x02 => DataType::Integer8,
            0x03 => DataType::Integer16,
            0x04 => DataType::Integer32,
            0x05 => DataType::Unsigned8,
            0x06 => DataType::Unsigned16,
            0x07 => DataType::Unsigned32,
            0x08 => DataType::Real32,
            0x09 => DataType::VisibleString,
            0x0A => DataType::OctetString,
            0x0F => DataType::Domain,
            0x11 => DataType::Real64,
            0x15 => DataType::Integer64,
            0x1B => DataType::Unsigned64,
            _ => return None,
        })
    }

    /// The fixed encoded size of a value of this type in bits, or `None` for a
    /// variable-length type (`VISIBLE_STRING`, `OCTET_STRING`, `DOMAIN`).
    pub const fn fixed_size_bits(self) -> Option<usize> {
        Some(match self {
            DataType::Boolean => 1,
            DataType::Integer8 | DataType::Unsigned8 => 8,
            DataType::Integer16 | DataType::Unsigned16 => 16,
            DataType::Integer32 | DataType::Unsigned32 | DataType::Real32 => 32,
            DataType::Integer64 | DataType::Unsigned64 | DataType::Real64 => 64,
            DataType::VisibleString
            | DataType::OctetString
            | DataType::Domain
            | DataType::PdoCommunicationParameter
            | DataType::PdoMapping
            | DataType::SdoParameter
            | DataType::Identity => return None,
        })
    }

    /// The fixed encoded size of a value of this type in bytes, or `None` for a
    /// variable-length type (`VISIBLE_STRING`, `OCTET_STRING`, `DOMAIN`).
    pub const fn fixed_size(self) -> Option<usize> {
        Some(match self {
            DataType::Boolean | DataType::Integer8 | DataType::Unsigned8 => 1,
            DataType::Integer16 | DataType::Unsigned16 => 2,
            DataType::Integer32 | DataType::Unsigned32 | DataType::Real32 => 4,
            DataType::Integer64 | DataType::Unsigned64 | DataType::Real64 => 8,
            DataType::VisibleString
            | DataType::OctetString
            | DataType::Domain
            | DataType::PdoCommunicationParameter
            | DataType::PdoMapping
            | DataType::SdoParameter
            | DataType::Identity => return None,
        })
    }

    /// Whether this is a variable-length type.
    pub const fn is_variable(self) -> bool {
        self.fixed_size_bits().is_none()
    }

    /// The fixed encoded size of a value of this type in bytes, or `0` for a
    /// variable-length type (whose size depends on the value — see
    /// [`fixed_size`](DataType::fixed_size)).
    pub const fn size(self) -> usize {
        match self.fixed_size() {
            Some(n) => n,
            None => 0,
        }
    }

    pub const fn bit_size(self) -> usize {
        match self.fixed_size_bits() {
            Some(n) => n,
            None => 0,
        }
    }
}
