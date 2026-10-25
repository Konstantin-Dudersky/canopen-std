use embedded_can::{Id, StandardId};

/// 0x1005: COB-ID SYNC
pub struct SyncMessageCobId {
    /// Default: 0x080
    pub cob_id: Id,
    pub device_generate_sync: bool,
}

impl SyncMessageCobId {
    pub const INDEX: u16 = 0x1005;
    pub const NAME: &'static str = "COB-ID SYNC";
    pub const COB_ID_DEFAULT: Id =
        Id::Standard(StandardId::new(0x080).expect("Invalid COB-ID for SYNC message"));
}

impl Default for SyncMessageCobId {
    fn default() -> Self {
        Self {
            cob_id: Self::COB_ID_DEFAULT,
            device_generate_sync: false,
        }
    }
}
