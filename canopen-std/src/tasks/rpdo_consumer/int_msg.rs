pub enum IntMsg {
    Sync,
    CanFrame(Vec<u8>),
}
