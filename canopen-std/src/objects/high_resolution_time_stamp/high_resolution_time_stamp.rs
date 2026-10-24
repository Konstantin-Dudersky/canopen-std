use std::time::Duration;

/// 0x1013 - High resolution time stamp
pub struct HighResolutionTimeStamp {
    pub time: Duration,
}

impl HighResolutionTimeStamp {
    pub const INDEX: u16 = 0x1013;
    pub const NAME: &str = "High resolution time stamp";
}
