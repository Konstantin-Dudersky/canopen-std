use std::time::Duration;

/// 0x1017: Producer heartbeat time
#[derive(Clone, Debug, Default)]
pub struct ProducerHeartbeatTime {
    pub heartbeat_time: Duration,
}

impl ProducerHeartbeatTime {
    pub const INDEX: u16 = 0x1017;
    pub const NAME: &str = "Producer heartbeat time";
}
