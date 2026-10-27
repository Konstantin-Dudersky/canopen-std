use crate::core::nmt::NmtCommand;
use crate::types::Value;

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
    ObjectDictdChange {
        index: u16,
        subindex: u8,
        value: Value,
    },
    SdoRead {
        node_id: u8,
        index: u16,
        subindex: u8,
    },
    SdoWrite {
        node_id: u8,
        index: u16,
        subindex: u8,
        value: Value,
    },
}
