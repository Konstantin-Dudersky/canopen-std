/// 0x1019: Synchronous counter overflow value
#[derive(Debug, Default)]
pub struct SyncCounterOverflow {
    /// Synchronous counter overflow value
    ///
    /// - 0: The SYNC message shall be transmitted as a CAN message of data length 0
    ///
    /// - 2 to 240: The SYNC message shall be transmitted as a CAN message of data length 1. The
    ///   first data byte contains the counter.
    pub counter: u8,
}

impl SyncCounterOverflow {
    pub const INDEX: u16 = 0x1019;
    pub const NAME: &'static str = "Synchronous counter overflow value";
}
