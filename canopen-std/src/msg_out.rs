use crate::{core::nmt::NmtState, Value};

#[derive(Debug)]
pub enum MsgOut {
    CanFrame {
        id: embedded_can::Id,
        data: Vec<u8>,
    },
    NmtStateChanged(NmtState),
    SyncReceived {
        counter: Option<u8>,
    },
    ObjectChanged {
        index: u16,
        subindex: u8,
        value: Value,
    },
}
