/// Access rights for an object dictionary entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessType {
    /// Read-only.
    Ro,
    /// Write-only.
    Wo,
    /// Read/write.
    Rw,
    /// Read-only constant (value fixed at design time).
    Const,
}

impl AccessType {
    /// Whether an SDO/PDO read is permitted.
    pub const fn is_readable(self) -> bool {
        matches!(self, AccessType::Ro | AccessType::Rw | AccessType::Const)
    }

    /// Whether an SDO/PDO write is permitted.
    pub const fn is_writable(self) -> bool {
        matches!(self, AccessType::Wo | AccessType::Rw)
    }
}
