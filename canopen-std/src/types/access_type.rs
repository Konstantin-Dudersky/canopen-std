#[derive(Debug, Clone, Copy)]
pub enum AccessType {
    Rw,
    Ro,
    Wo,
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
}
