use embedded_can::{Id, StandardId};

/// 0x1012 - COB-ID time stamp object
pub struct TimeStampCobId {
    pub cob_id: Id,
    pub device_produce_time: bool,
    pub device_consume_time: bool,
}

impl TimeStampCobId {
    pub const INDEX: u16 = 0x1012;
    pub const COB_ID_DEFAULT: Id =
        Id::Standard(StandardId::new(0x100).expect("invalid standard id"));

    pub const NAME: &'static str = "COB-ID time stamp object";
}

impl Default for TimeStampCobId {
    fn default() -> Self {
        Self {
            cob_id: Self::COB_ID_DEFAULT,
            device_produce_time: false,
            device_consume_time: false,
        }
    }
}
