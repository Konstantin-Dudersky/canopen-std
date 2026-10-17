use std::fmt;

/// Стандартные типы данных CANopen (CiA 301, раздел 7.4.7).
/// Значение варианта — индекс типа в словаре объектов,
/// он же записывается в EDS как `DataType=0x....`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    Boolean,
    Integer8,
    Integer16,
    Integer32,
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Real32,
    VisibleString,
    OctetString,
    UnicodeString,
    TimeOfDay,
    TimeDifference,
    Domain,
    Integer24,
    Real64,
    Integer40,
    Integer48,
    Integer56,
    Integer64,
    Unsigned24,
    Unsigned40,
    Unsigned48,
    Unsigned56,
    Unsigned64,
    PdoCommunicationParameter,
    PdoMapping,
    SdoParameter,
    Identity,
}

impl DataType {
    /// Индекс типа в словаре объектов.
    pub const fn index(self) -> u16 {
        match self {
            DataType::Boolean => 0x0001,
            DataType::Integer8 => 0x0002,
            DataType::Integer16 => 0x0003,
            DataType::Integer32 => 0x0004,
            DataType::Unsigned8 => 0x0005,
            DataType::Unsigned16 => 0x0006,
            DataType::Unsigned32 => 0x0007,
            DataType::Real32 => 0x0008,
            DataType::VisibleString => 0x0009,
            DataType::OctetString => 0x000A,
            DataType::UnicodeString => 0x000B,
            DataType::TimeOfDay => 0x000C,
            DataType::TimeDifference => 0x000D,
            DataType::Domain => 0x000F,
            DataType::Integer24 => 0x0010,
            DataType::Real64 => 0x0011,
            DataType::Integer40 => 0x0012,
            DataType::Integer48 => 0x0013,
            DataType::Integer56 => 0x0014,
            DataType::Integer64 => 0x0015,
            DataType::Unsigned24 => 0x0016,
            DataType::Unsigned40 => 0x0018,
            DataType::Unsigned48 => 0x0019,
            DataType::Unsigned56 => 0x001A,
            DataType::Unsigned64 => 0x001B,
            DataType::PdoCommunicationParameter => 0x0020,
            DataType::PdoMapping => 0x0021,
            DataType::SdoParameter => 0x0022,
            DataType::Identity => 0x0023,
        }
    }

    /// Размер в битах для простых типов фиксированной длины.
    /// `None` — для типов переменной длины (строки, DOMAIN) и структур.
    pub const fn bit_size(self) -> Option<u32> {
        use DataType::*;
        match self {
            Boolean => Some(1),
            Integer8 | Unsigned8 => Some(8),
            Integer16 | Unsigned16 => Some(16),
            Integer24 | Unsigned24 => Some(24),
            Integer32 | Unsigned32 | Real32 => Some(32),
            Integer40 | Unsigned40 => Some(40),
            Integer48 | Unsigned48 | TimeOfDay | TimeDifference => Some(48),
            Integer56 | Unsigned56 => Some(56),
            Integer64 | Unsigned64 | Real64 => Some(64),
            VisibleString | OctetString | UnicodeString | Domain => None,
            PdoCommunicationParameter | PdoMapping | SdoParameter | Identity => None,
        }
    }

    /// Простой (не составной) тип — может быть `DataType` у объекта VAR
    /// и элементов ARRAY/RECORD.
    pub const fn is_basic(self) -> bool {
        !matches!(
            self,
            DataType::PdoCommunicationParameter
                | DataType::PdoMapping
                | DataType::SdoParameter
                | DataType::Identity
        )
    }

    /// Имя типа по CiA 301
    pub const fn name(self) -> &'static str {
        use DataType::*;
        match self {
            Boolean => "BOOLEAN",
            Integer8 => "INTEGER8",
            Integer16 => "INTEGER16",
            Integer32 => "INTEGER32",
            Unsigned8 => "UNSIGNED8",
            Unsigned16 => "UNSIGNED16",
            Unsigned32 => "UNSIGNED32",
            Real32 => "REAL32",
            VisibleString => "VISIBLE_STRING",
            OctetString => "OCTET_STRING",
            UnicodeString => "UNICODE_STRING",
            TimeOfDay => "TIME_OF_DAY",
            TimeDifference => "TIME_DIFFERENCE",
            Domain => "DOMAIN",
            Integer24 => "INTEGER24",
            Real64 => "REAL64",
            Integer40 => "INTEGER40",
            Integer48 => "INTEGER48",
            Integer56 => "INTEGER56",
            Integer64 => "INTEGER64",
            Unsigned24 => "UNSIGNED24",
            Unsigned40 => "UNSIGNED40",
            Unsigned48 => "UNSIGNED48",
            Unsigned56 => "UNSIGNED56",
            Unsigned64 => "UNSIGNED64",
            PdoCommunicationParameter => "PDO_COMMUNICATION_PARAMETER",
            PdoMapping => "PDO_MAPPING",
            SdoParameter => "SDO_PARAMETER",
            Identity => "IDENTITY",
        }
    }
}

/// Форматирование в виде значения для EDS: `0x0005`.
impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self.index())
    }
}
