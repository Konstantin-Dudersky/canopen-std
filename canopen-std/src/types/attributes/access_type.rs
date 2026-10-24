use crate::eds::EdsError;

/// Access rights for an object dictionary entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessType {
    /// Read/write.
    Rw,

    /// Read-only.
    Ro,

    /// Write-only.
    Wo,

    /// Read-only constant (value fixed at design time).
    Const,

    /// For TPDO
    Rwr,

    /// For RPDO
    Rww,
}

impl AccessType {
    pub fn to_eds(&self) -> &'static str {
        match self {
            AccessType::Rw => "rw",
            AccessType::Ro => "ro",
            AccessType::Wo => "wo",
            AccessType::Const => "const",
            AccessType::Rwr => "rwr",
            AccessType::Rww => "rww",
        }
    }

    /// Whether an SDO/PDO read is permitted.
    pub const fn is_readable(self) -> bool {
        matches!(self, AccessType::Ro | AccessType::Rw | AccessType::Const)
    }

    /// Whether an SDO/PDO write is permitted.
    pub const fn is_writable(self) -> bool {
        matches!(self, AccessType::Wo | AccessType::Rw)
    }
}

impl TryFrom<&str> for AccessType {
    type Error = EdsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_ascii_lowercase().as_str() {
            "ro" => Ok(AccessType::Ro),
            "wo" => Ok(AccessType::Wo),
            "rw" | "rwr" | "rww" => Ok(AccessType::Rw),
            "const" => Ok(AccessType::Const),
            other => Err(EdsError::UnknownAccessType(other.to_string())),
        }
    }
}
