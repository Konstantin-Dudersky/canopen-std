use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};
use tracing::info;

use crate::{
    Error, MsgOut,
    core::nmt::{BOOTUP_FRAME, NmtState, NmtStateMachine, decode_command, heartbeat_cob_id},
    types::NodeId,
};

use super::int_msg::IntMsg;

pub struct TaskMain {
    pub input: mpsc::Receiver<IntMsg>,
    pub output: mpsc::Sender<MsgOut>,
    pub nmt_state: Arc<Mutex<NmtStateMachine>>,
    pub node_id: NodeId,
}

impl TaskMain {
    pub async fn run(mut self) -> Result<(), Error> {
        while let Some(msg) = self.input.recv().await {
            match msg {
                IntMsg::CanFrameReceived(data) => {
                    if data.len() < 2 {
                        continue;
                    }
                    if let Ok((command, target)) = decode_command(&[data[0], data[1]]) {
                        if target == NodeId::BROADCAST || target == self.node_id {
                            let mut nmt_state = self.nmt_state.lock().await;
                            let new_state = nmt_state.apply(command);
                            let msg_out = MsgOut::NmtStateChanged(new_state);
                            self.output.send(msg_out).await.unwrap();
                        }
                    }
                }
                IntMsg::PeriodicCheck => {
                    let mut nmt_state = self.nmt_state.lock().await;
                    if nmt_state.state() == NmtState::Initialising {
                        let new_state = nmt_state.boot();
                        info!("Node booted");

                        let msg_out = MsgOut::CanFrame {
                            id: heartbeat_cob_id(self.node_id),
                            data: BOOTUP_FRAME.to_vec(),
                        };
                        self.output.send(msg_out).await.unwrap();

                        let msg_out = MsgOut::NmtStateChanged(new_state);
                        self.output.send(msg_out).await.unwrap();
                    }
                }
                IntMsg::HeartbeatConsumerCommand(nc) => {
                    let mut nmt_state = self.nmt_state.lock().await;
                    let new_state = nmt_state.apply(nc);
                    let msg_out = MsgOut::NmtStateChanged(new_state);
                    self.output.send(msg_out).await.unwrap();
                }
            }
        }
        Ok(())
    }
}
