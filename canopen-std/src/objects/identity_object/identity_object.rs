/// 0x1018: Identity object
pub struct IdentityObject {
    pub vendor_id: u32,
    pub product_code: u32,
    pub revision_number: u32,
    pub serial_number: u32,
}

impl IdentityObject {
    pub const INDEX: u16 = 0x1018;
    pub const NAME: &str = "Identity object";
}
