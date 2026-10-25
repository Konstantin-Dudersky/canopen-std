/// 0x1001: Error register
#[derive(Clone, Debug, Default)]
pub struct ErrorRegister {
    pub generic_error: bool,
    pub current: bool,
    pub voltage: bool,
    pub temperature: bool,
    pub communication_error: bool,
    pub device_profile_specific: bool,
    pub manufacturer_specific: bool,
}

impl ErrorRegister {
    pub const INDEX: u16 = 0x1001;
    pub const NAME: &'static str = "Error register";
}
