/// 0x1008: Manufacturer device name
#[derive(Clone, Debug, Default)]
pub struct ManufacturerDeviceName {
    pub name: String,
}

impl ManufacturerDeviceName {
    pub const INDEX: u16 = 0x1008;
    pub const NAME: &str = "Manufacturer device name";
}
