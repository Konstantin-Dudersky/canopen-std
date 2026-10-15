#[derive(Clone, Copy, Debug)]
pub enum IntMsg {
    Sync(Option<u8>),
    OdChanged,
    EventTimer,
    InhibitTimer,
}
