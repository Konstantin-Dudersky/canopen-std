use crate::types::Value;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // #[error("Convert output can frame failed")]
    // ConvertOutputCanFrame(crate::core::node::TxFrame),
    #[error("Error opening EDS file: {0}")]
    Edserror(#[from] crate::eds::EdsError),

    // #[error("Error from canopen_rs: {0}")]
    // CanopenRs(#[from] canopen_rs::Error),
    #[error("fn_input failed")]
    FnInput(anyhow::Error),

    #[error("mpsc send failed in {0}")]
    MpscSend(&'static str),

    #[error("Task end: {0}")]
    TaskEnd(&'static str),

    #[error("Wrong datatype of value in OD. {description}: {value:?}")]
    WrongDatatype {
        description: &'static str,
        value: Value,
    },

    /// A frame or buffer was too short or too long for the operation.
    #[error("Bad length")]
    BadLength,

    /// A value did not fit the target CANopen data type.
    #[error("Overflow")]
    Overflow,

    #[error("Tokio task ended. Name: {description}")]
    TaskEnded { description: &'static str },

    /// No object exists at the requested (index, subindex).
    #[error("Object not found")]
    ObjectNotFound,

    /// No entry exists at the requested (index, subindex).
    #[error("Entry not found. Index: {index}, subindex: {subindex}")]
    EntryNotFound { index: u16, subindex: u8 },

    /// A node id outside the valid `1..=127` range was supplied.
    #[error("Invalid node id")]
    InvalidNodeId,

    /// Invalid COB ID.
    #[error("Invalid cob id")]
    InvalidCobId,
    /// Attempt to write an object that is not writable.
    #[error("Read only")]
    ReadOnly,
    /// Attempt to read an object that is not readable.
    #[error("Write only")]
    WriteOnly,
    /// A value's data type did not match the object's data type.
    #[error("Type mismatch")]
    TypeMismatch,

    /// A value's data type did not match the object's data type.
    #[error("Type mismatch. Value: {value:?}, location: {location}")]
    TypeMismatch2 { value: Value, location: String },

    /// The value cannot be carried by the chosen transfer (e.g. a value
    /// larger than four bytes attempted over expedited SDO).
    #[error("Unsupported transfer")]
    UnsupportedTransfer,
    /// A received frame's command byte was not the expected response.
    #[error("Unexpected command")]
    UnexpectedCommand,
    /// A heartbeat frame carried an unrecognised NMT state value.
    #[error("Unknown state")]
    UnknownState,
    /// A PDO's mapped objects total more than eight bytes (64 bits).
    #[error("Pdo too long")]
    PdoTooLong,
    /// A PDO mapping's fixed capacity is exhausted.
    #[error("Mapping full")]
    MappingFull,
    /// A SYNC counter overflow value outside `0` or `2..=240`.
    #[error("Invalid sync counter")]
    InvalidSyncCounter,
    /// An SDO segment's toggle bit did not alternate as required.
    #[error("Toggle mismatch")]
    ToggleMismatch,
    /// A block-transfer CRC did not match the received data.
    #[error("Crc mismatch")]
    CrcMismatch,
}
