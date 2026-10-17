use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{mpsc, Mutex},
    time::sleep,
};

use crate::{
    core::{nmt::NmtCommand, object_dictionary::objects_old::object_0x1029::Object0x1029ErrorBehavior},
    Error,
};

use super::HeartbeatConsumerState;

pub struct TaskLoop {
    pub error_behavior: Object0x1029ErrorBehavior,
    pub output_nmt_slave: mpsc::Sender<NmtCommand>,
    pub heartbeat_consumer_state: Arc<Mutex<HeartbeatConsumerState>>,
}

impl TaskLoop {
    pub async fn run(self) -> Result<(), Error> {
        loop {
            let timeout_occured = self.heartbeat_consumer_state.lock().await.timeout_occured();

            if timeout_occured {
                match self.error_behavior {
                    Object0x1029ErrorBehavior::EnterPreOperational => {
                        let cmd = NmtCommand::EnterPreOperational;
                        self.output_nmt_slave.send(cmd).await.unwrap();
                    }
                    Object0x1029ErrorBehavior::NoChange => (),
                    Object0x1029ErrorBehavior::Stop => {
                        let cmd = NmtCommand::StopRemoteNode;
                        self.output_nmt_slave.send(cmd).await.unwrap();
                    }
                }
            }

            sleep(Duration::from_millis(100)).await
        }
    }
}
