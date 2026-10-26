//! Network Management (NMT) — the node state machine (CiA 301 §7.3).
//!
//! NMT governs a node's lifecycle. A node powers up in *Initialisation*,
//! emits its boot-up message, and autonomously enters *Pre-operational*.
//! From there an NMT master moves it between *Operational* and *Stopped* with
//! node-control commands broadcast on COB-ID `0x000`. Each node reports its
//! current state to the network through the error-control (heartbeat)
//! producer on `0x700 + node`.
//!
//! ```text
//!                    ┌──────────────── reset ◄────────────┐
//!                    ▼                                     │
//!            ┌───────────────┐  boot   ┌──────────────────┴─┐
//!            │ Initialising  ├────────►│  Pre-operational   │
//!            └───────────────┘         └──┬──────────────▲──┘
//!                              start(0x01)│              │enter-preop(0x80)
//!                                         ▼              │
//!                                   ┌─────┴──────────────┴──┐
//!                       ┌──────────►│     Operational       │
//!            start(0x01)│           └─────────┬─────────────┘
//!                       │              stop(0x02)
//!                 ┌─────┴─────┐               ▼
//!                 │  Stopped  │◄──────────────┘
//!                 └───────────┘
//! ```
//!
//! Like the SDO codec, the functions here encode and decode raw CAN *data
//! fields* only; selecting the COB-ID and moving the frame is the transport's
//! job. [`NmtStateMachine`] is the node-side state logic, with no I/O.
//!
//! ```
//! use canopen_rs::nmt::encode_command;
//! use canopen_rs::{NmtCommand, NmtState, NmtStateMachine, NodeId};
//!
//! // Master side: command node 5 to start (enter operational).
//! let frame = encode_command(NmtCommand::StartRemoteNode, NodeId::new(5).unwrap());
//! assert_eq!(frame, [0x01, 0x05]); // [command specifier, target node]
//!
//! // Node side: the guarded state machine.
//! let mut sm = NmtStateMachine::new();
//! assert_eq!(sm.state(), NmtState::Initialising);
//! sm.boot(); // -> pre-operational
//! assert_eq!(sm.apply(NmtCommand::StartRemoteNode), NmtState::Operational);
//! assert_eq!(sm.apply(NmtCommand::ResetNode), NmtState::Initialising);
//! ```

mod nmt_state;
mod nmt_state_machine;

use embedded_can::{Id, StandardId};

pub use {nmt_state::NmtState, nmt_state_machine::NmtStateMachine};

use crate::{Error, types::NodeId};

/// COB-ID of the NMT node-control channel (master → nodes): `0x000`.
///
/// This is the highest-priority id on the bus; a single frame can address one
/// node or, with target `0`, every node at once.
pub const NMT_COMMAND_COB_ID: Id = Id::Standard(StandardId::new(0x000).unwrap());
pub const NMT_COMMAND_COB_ID_OLD: u16 = 0x000;

/// COB-ID base for the error-control (heartbeat / boot-up) channel:
/// `0x700 + node`.
pub const HEARTBEAT_COB_BASE: u16 = 0x700;

/// The COB-ID of a node's heartbeat / boot-up producer channel.
pub const fn heartbeat_cob_id(node: NodeId) -> Id {
    let id_raw = HEARTBEAT_COB_BASE + node.raw() as u16;
    Id::Standard(StandardId::new(id_raw).unwrap())
}

/// An NMT node-control command from the master (CiA 301 §7.2.8.2).
///
/// The discriminant is the command specifier carried in byte 0 of an
/// `0x000` frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NmtCommand {
    /// Start remote node → *Operational*.
    StartRemoteNode = 0x01,
    /// Stop remote node → *Stopped*.
    StopRemoteNode = 0x02,
    /// Enter pre-operational → *Pre-operational*.
    EnterPreOperational = 0x80,
    /// Reset node → re-enter *Initialising* (application + communication).
    ResetNode = 0x81,
    /// Reset communication → re-enter *Initialising* (communication only).
    ResetCommunication = 0x82,
}

impl NmtCommand {
    /// Decode a command specifier byte.
    ///
    /// Returns [`Error::UnexpectedCommand`] for an unrecognised specifier.
    pub const fn from_byte(byte: u8) -> Result<Self, Error> {
        Ok(match byte {
            0x01 => NmtCommand::StartRemoteNode,
            0x02 => NmtCommand::StopRemoteNode,
            0x80 => NmtCommand::EnterPreOperational,
            0x81 => NmtCommand::ResetNode,
            0x82 => NmtCommand::ResetCommunication,
            _ => return Err(Error::UnexpectedCommand),
        })
    }
}

/// The 2-byte data field of an NMT node-control frame.
pub type NmtCommandFrame = [u8; 2];

/// The 1-byte data field of a heartbeat / boot-up frame.
pub type HeartbeatFrame = [u8; 1];

/// The boot-up message data field (`0x00`), emitted once when a node leaves
/// initialisation and enters pre-operational.
pub const BOOTUP_FRAME: HeartbeatFrame = [0x00];

/// Encode an NMT node-control frame for `target`.
///
/// Pass [`NodeId::BROADCAST`] to address every node on the bus at once.
pub fn encode_command(command: NmtCommand, target: NodeId) -> NmtCommandFrame {
    [command as u8, target.raw()]
}

/// Decode an NMT node-control frame into `(command, target)`.
///
/// A target byte of `0` decodes to [`NodeId::BROADCAST`]; any other value is a
/// checked device id (`1..=127`), else [`Error::InvalidNodeId`].
pub fn decode_command(frame: &NmtCommandFrame) -> Result<(NmtCommand, NodeId), Error> {
    let command = NmtCommand::from_byte(frame[0])?;
    let target = if frame[1] == 0 {
        NodeId::BROADCAST
    } else {
        NodeId::new(frame[1])?
    };
    Ok((command, target))
}

/// Encode a heartbeat frame reporting `state`.
pub fn encode_heartbeat(state: NmtState) -> HeartbeatFrame {
    [state as u8]
}

/// Decode a heartbeat frame into the reported [`NmtState`].
pub fn decode_heartbeat(frame: &HeartbeatFrame) -> Result<NmtState, Error> {
    NmtState::from_wire(frame[0])
}

/// Encode a **node-guarding** response byte: the alternating toggle bit in bit
/// 7 and the current [`NmtState`] in bits 0–6.
///
/// Node guarding (CiA 301 §7.3.1) is the legacy error-control alternative to
/// heartbeat: the master polls the node with an RTR frame on `0x700 + node` and
/// the node replies with this byte, toggling bit 7 on each reply so lost or
/// duplicated responses are detectable.
pub fn encode_node_guard(state: NmtState, toggle: bool) -> u8 {
    (if toggle { 0x80 } else { 0 }) | state as u8
}

/// Decode a node-guarding response byte into `(toggle, state)`.
pub fn decode_node_guard(byte: u8) -> Result<(bool, NmtState), Error> {
    Ok((byte & 0x80 != 0, NmtState::from_wire(byte)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- COB-IDs -----------------------------------------------------------
    #[test]
    fn heartbeat_cob_id_follows_convention() {
        let id: Id = StandardId::new(0x705).unwrap().into();
        assert_eq!(heartbeat_cob_id(NodeId::new(0x05).unwrap()), id);

        assert_eq!(
            NMT_COMMAND_COB_ID,
            Id::Standard(StandardId::new(0x000).unwrap())
        );
    }

    // --- Command codec: known-good frames ----------------------------------
    // NMT frames are [command specifier, target node]. Target 0 = all nodes.
    #[test]
    fn start_broadcast_matches_known_frame() {
        assert_eq!(
            encode_command(NmtCommand::StartRemoteNode, NodeId::BROADCAST),
            [0x01, 0x00]
        );
    }

    #[test]
    fn start_single_node_matches_known_frame() {
        let f = encode_command(NmtCommand::StartRemoteNode, NodeId::new(5).unwrap());
        assert_eq!(f, [0x01, 0x05]);
    }

    #[test]
    fn stop_single_node_matches_known_frame() {
        let f = encode_command(NmtCommand::StopRemoteNode, NodeId::new(5).unwrap());
        assert_eq!(f, [0x02, 0x05]);
    }

    #[test]
    fn reset_commands_match_known_frames() {
        let node = NodeId::new(0x7F).unwrap();
        assert_eq!(
            encode_command(NmtCommand::EnterPreOperational, node),
            [0x80, 0x7F]
        );
        assert_eq!(encode_command(NmtCommand::ResetNode, node), [0x81, 0x7F]);
        assert_eq!(
            encode_command(NmtCommand::ResetCommunication, node),
            [0x82, 0x7F]
        );
    }

    #[test]
    fn command_frame_roundtrips() {
        let (cmd, target) = decode_command(&[0x01, 0x05]).unwrap();
        assert_eq!(cmd, NmtCommand::StartRemoteNode);
        assert_eq!(target, NodeId::new(5).unwrap());

        let (cmd, target) = decode_command(&[0x82, 0x00]).unwrap();
        assert_eq!(cmd, NmtCommand::ResetCommunication);
        assert_eq!(target, NodeId::BROADCAST);
    }

    #[test]
    fn decode_rejects_unknown_command() {
        assert!(matches!(
            decode_command(&[0x7E, 0x05]),
            Err(Error::UnexpectedCommand)
        ));
    }

    // --- Heartbeat / boot-up codec: known-good frames ----------------------
    #[test]
    fn heartbeat_states_match_known_frames() {
        assert_eq!(encode_heartbeat(NmtState::Operational), [0x05]);
        assert_eq!(encode_heartbeat(NmtState::Stopped), [0x04]);
        assert_eq!(encode_heartbeat(NmtState::PreOperational), [0x7F]);
        assert_eq!(BOOTUP_FRAME, [0x00]);
    }

    #[test]
    fn heartbeat_decode_ignores_toggle_bit() {
        // Bit 7 is reserved (toggle in node guarding); decoding must mask it.
        assert_eq!(decode_heartbeat(&[0x05]).unwrap(), NmtState::Operational);
        assert_eq!(decode_heartbeat(&[0x85]).unwrap(), NmtState::Operational);
    }

    #[test]
    fn heartbeat_decode_rejects_unknown_state() {
        assert!(matches!(
            decode_heartbeat(&[0x42]),
            Err(Error::UnknownState)
        ));
    }

    // --- Node guarding -----------------------------------------------------
    #[test]
    fn node_guard_encodes_toggle_and_state() {
        // Operational (0x05) with toggle clear, then set.
        assert_eq!(encode_node_guard(NmtState::Operational, false), 0x05);
        assert_eq!(encode_node_guard(NmtState::Operational, true), 0x85);
        assert_eq!(
            decode_node_guard(0x85).unwrap(),
            (true, NmtState::Operational)
        );
        assert_eq!(
            decode_node_guard(0x7F).unwrap(),
            (false, NmtState::PreOperational)
        );
        assert!(matches!(decode_node_guard(0x42), Err(Error::UnknownState)));
    }

    // --- State machine -----------------------------------------------------
    #[test]
    fn boots_from_init_to_preoperational() {
        let mut sm = NmtStateMachine::new();
        assert_eq!(sm.state(), NmtState::Initialising);
        assert_eq!(sm.boot(), NmtState::PreOperational);
    }

    #[test]
    fn ignores_operational_commands_while_initialising() {
        let mut sm = NmtStateMachine::new();
        assert_eq!(
            sm.apply(NmtCommand::StartRemoteNode),
            NmtState::Initialising
        );
    }

    #[test]
    fn full_operational_lifecycle() {
        let mut sm = NmtStateMachine::new();
        sm.boot();
        assert_eq!(sm.apply(NmtCommand::StartRemoteNode), NmtState::Operational);
        assert_eq!(
            sm.apply(NmtCommand::EnterPreOperational),
            NmtState::PreOperational
        );
        assert_eq!(sm.apply(NmtCommand::StopRemoteNode), NmtState::Stopped);
        assert_eq!(sm.apply(NmtCommand::StartRemoteNode), NmtState::Operational);
    }

    #[test]
    fn reset_returns_to_initialising_from_any_state() {
        let mut sm = NmtStateMachine::new();
        sm.boot();
        sm.apply(NmtCommand::StartRemoteNode);
        assert_eq!(sm.apply(NmtCommand::ResetNode), NmtState::Initialising);
        // And a fresh boot works again.
        assert_eq!(sm.boot(), NmtState::PreOperational);

        sm.apply(NmtCommand::StartRemoteNode);
        assert_eq!(
            sm.apply(NmtCommand::ResetCommunication),
            NmtState::Initialising
        );
    }

    #[test]
    fn repeated_command_is_idempotent() {
        let mut sm = NmtStateMachine::new();
        sm.boot();
        sm.apply(NmtCommand::StartRemoteNode);
        assert_eq!(sm.apply(NmtCommand::StartRemoteNode), NmtState::Operational);
    }
}
