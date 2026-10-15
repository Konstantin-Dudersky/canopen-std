use crate::Error;

/// A node's NMT state.
///
/// The discriminant is the value carried in a heartbeat frame (CiA 301
/// §7.2.8.3). [`NmtState::Initialising`] is reported as `0` — the value of a
/// boot-up message, sent once as the node leaves initialisation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NmtState {
    /// Initialising (boot-up); reported as `0`.
    Initialising = 0x00,
    /// Stopped: only NMT and error-control are active.
    Stopped = 0x04,
    /// Operational: all communication objects (incl. PDOs) are active.
    Operational = 0x05,
    /// Pre-operational: SDOs active, PDOs inactive.
    PreOperational = 0x7F,
}

impl NmtState {
    /// Decode a heartbeat state byte, ignoring the reserved toggle bit (7).
    ///
    /// Returns [`Error::UnknownState`] for an unrecognised value.
    pub const fn from_wire(byte: u8) -> Result<Self, Error> {
        Ok(match byte & 0x7F {
            0x00 => NmtState::Initialising,
            0x04 => NmtState::Stopped,
            0x05 => NmtState::Operational,
            0x7F => NmtState::PreOperational,
            _ => return Err(Error::UnknownState),
        })
    }
}
