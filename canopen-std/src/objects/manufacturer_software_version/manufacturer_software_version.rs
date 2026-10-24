/// 0x1009: Manufacturer hardware version
pub struct ManufacturerSoftwareVersion {
    pub name: String,
}

impl ManufacturerSoftwareVersion {
    pub const INDEX: u16 = 0x100A;
    pub const NAME: &str = "Manufacturer software version";
}
