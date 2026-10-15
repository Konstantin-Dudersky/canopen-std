mod canopen_node;
mod core;
mod error;
mod msg_in;
mod msg_out;
mod tasks;

/// Electronic Data Sheet (EDS) / Device Configuration File (DCF) parsing.
///
/// Cross-platform: EDS files are plain text and carry no OS dependency.
mod eds;

pub use {
    canopen_node::CANopenNode,
    core::datatypes::{DataType, Value},
    core::nmt::NmtCommand,
    core::object_dictionary::objects,
    core::object_dictionary::{AccessType, Address, Entry},
    error::Error,
    msg_in::MsgIn,
    msg_out::MsgOut,
};
