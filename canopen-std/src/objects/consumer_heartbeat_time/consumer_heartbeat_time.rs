use super::ConsumerHeartbeatTimeItem;

/// 0x1016: Consumer heartbeat time
#[derive(Debug)]
pub struct ConsumerHeartbeatTime {
    pub items: Vec<ConsumerHeartbeatTimeItem>,
}

impl ConsumerHeartbeatTime {
    pub const INDEX: u16 = 0x1016;
    pub const NAME: &str = "Consumer heartbeat time";
}
