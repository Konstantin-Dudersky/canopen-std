use std::sync::Arc;

use canopen_rs::NodeId;
use embedded_can::Id;
use tokio::sync::{mpsc, Mutex};

use crate::{core::nmt::HEARTBEAT_COB_BASE, Error};

use super::HeartbeatConsumerState;

pub struct TaskInput {
    pub input: mpsc::Receiver<Id>,
    pub heartbeat_consumer_state: Arc<Mutex<HeartbeatConsumerState>>,
}

impl TaskInput {
    pub async fn run(mut self) -> Result<(), Error> {
        while let Some(id) = self.input.recv().await {
            let node_id = match id {
                Id::Standard(id) => id.as_raw().checked_sub(HEARTBEAT_COB_BASE).unwrap() as u8,
                Id::Extended(_) => continue,
            };
            let node_id = NodeId::new(node_id).ok().unwrap();

            self.heartbeat_consumer_state.lock().await.on_frame(node_id);
        }
        Ok(())
    }
}
