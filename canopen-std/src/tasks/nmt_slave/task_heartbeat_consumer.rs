use tokio::sync::mpsc;

use crate::{Error, NmtCommand};

use super::int_msg::IntMsg;

pub struct TaskHeartbeatConsumer {
    pub input_heartbeat_consumer: mpsc::Receiver<NmtCommand>,
    pub output: mpsc::Sender<IntMsg>,
}

impl TaskHeartbeatConsumer {
    pub async fn run(mut self) -> Result<(), Error> {
        while let Some(heartbeat) = self.input_heartbeat_consumer.recv().await {
            self.output
                .send(IntMsg::HeartbeatConsumerCommand(heartbeat))
                .await
                .unwrap();
        }
        Ok(())
    }
}
