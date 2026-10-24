/// 0x1000: Device type
pub struct DeviceType {
    pub device_type: u32,
}

impl DeviceType {
    pub const INDEX: u16 = 0x1000;
    pub const NAME: &str = "Device Type";
}
