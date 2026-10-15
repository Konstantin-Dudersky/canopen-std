use super::{NmtCommand, NmtState};

/// The node-side NMT state machine (CiA 301 §7.3.2).
///
/// Holds only the current state and applies the guarded transitions of the
/// diagram above. It performs no I/O: feed it decoded [`NmtCommand`]s and read
/// back the resulting [`NmtState`] to drive heartbeat production and to gate
/// which services (e.g. PDOs) are active.
#[derive(Debug)]
pub struct NmtStateMachine {
    state: NmtState,
}

impl Default for NmtStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl NmtStateMachine {
    /// A freshly powered node, in [`NmtState::Initialising`].
    pub const fn new() -> Self {
        Self {
            state: NmtState::Initialising,
        }
    }

    /// The current state.
    pub const fn state(&self) -> NmtState {
        self.state
    }

    /// Complete initialisation: the node enters pre-operational (CiA 301
    /// §7.3.2.1). The caller emits [`BOOTUP_FRAME`] alongside this transition.
    ///
    /// A no-op unless currently initialising.
    pub fn boot(&mut self) -> NmtState {
        if let NmtState::Initialising = self.state {
            self.state = NmtState::PreOperational;
        }
        self.state
    }

    /// Apply an NMT master command, returning the resulting state.
    ///
    /// Transitions are guarded: a reset returns to *Initialising* from any
    /// state, while start/stop/enter-pre-operational are ignored during
    /// initialisation (the node is not yet a network participant) and
    /// otherwise move between the operational states. Repeating a command that
    /// matches the current state is a harmless no-op.
    pub fn apply(&mut self, command: NmtCommand) -> NmtState {
        self.state = match (self.state, command) {
            // Reset re-enters initialisation from any state.
            (_, NmtCommand::ResetNode | NmtCommand::ResetCommunication) => NmtState::Initialising,
            // During initialisation the node ignores operational commands; it
            // leaves this state only via `boot`.
            (NmtState::Initialising, _) => NmtState::Initialising,
            (_, NmtCommand::StartRemoteNode) => NmtState::Operational,
            (_, NmtCommand::StopRemoteNode) => NmtState::Stopped,
            (_, NmtCommand::EnterPreOperational) => NmtState::PreOperational,
        };
        self.state
    }

    pub fn is_sdo_server_enabled(&self) -> bool {
        matches!(self.state, NmtState::PreOperational | NmtState::Operational)
    }
}
