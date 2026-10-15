use crate::NmtCommand;

pub(crate) enum IntMsg {
    CanFrameReceived(Vec<u8>),
    PeriodicCheck,
    HeartbeatConsumerCommand(NmtCommand),
}
