/// 0x1009: Manufacturer hardware version
pub struct ManufacturerHardwareVersion {
    pub name: String,
}

impl ManufacturerHardwareVersion {
    pub const INDEX: u16 = 0x1009;
    pub const NAME: &str = "Manufacturer hardware version";
}
