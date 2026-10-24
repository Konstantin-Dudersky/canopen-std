use std::time::Duration;

/// 0x1006: Communication cycle period
#[derive(Clone, Debug, Default)]
pub struct CommCyclePeriod {
    pub period: Duration,
}

impl CommCyclePeriod {
    pub const INDEX: u16 = 0x1006;
    pub const NAME: &str = "Communication cycle period";
}
