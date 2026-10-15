use crate::core::{datatypes::Value, nmt::NmtCommand};

#[derive(Debug)]
pub enum MsgIn {
    CanFrame {
        id: embedded_can::Id,
        data: Vec<u8>,
    },
    NmtMasterCommand {
        target: u8,
        command: NmtCommand,
    },
    OdChange {
        index: u16,
        subindex: u8,
        value: Value,
    },
}
