mod int_msg;
mod task_heartbeat_consumer;
mod task_input;
mod task_main;

use std::{sync::Arc, time::Duration};

use canopen_rs::NodeId;
use tokio::{
    sync::{mpsc, Mutex},
    task::JoinSet,
    time::sleep,
};
use tracing::warn;

use crate::{core::nmt::NmtStateMachine, Error, MsgOut, NmtCommand};

use {task_heartbeat_consumer::TaskHeartbeatConsumer, task_input::TaskInput, task_main::TaskMain};

pub struct TaskNmtSlave {
    pub input_can_frame: mpsc::Receiver<Vec<u8>>,
    pub input_heartbeat_consumer: mpsc::Receiver<NmtCommand>,
    pub output: mpsc::Sender<MsgOut>,
    pub nmt_state: Arc<Mutex<NmtStateMachine>>,
    pub node_id: NodeId,
}

impl TaskNmtSlave {
    pub async fn run(self) -> Result<(), Error> {
        let (tx, rx) = mpsc::channel::<int_msg::IntMsg>(100);
        let mut local_task_set = JoinSet::new();

        let task = SubTaskCheckStateBoot {
            output: tx.clone(),
            periodic_check_interval: Duration::from_millis(100),
        };
        local_task_set.spawn(task.run());

        let task = TaskMain {
            input: rx,
            output: self.output.clone(),
            nmt_state: self.nmt_state.clone(),
            node_id: self.node_id,
        };
        local_task_set.spawn(task.run());

        let task = TaskInput {
            input: self.input_can_frame,
            output: tx.clone(),
        };
        local_task_set.spawn(task.run());

        let task = TaskHeartbeatConsumer {
            input_heartbeat_consumer: self.input_heartbeat_consumer,
            output: tx,
        };
        local_task_set.spawn(task.run());

        while let Some(result) = local_task_set.join_next().await {
            warn!("Result: {:?}", result);
        }

        Ok(())
    }
}

struct SubTaskCheckStateBoot {
    pub output: mpsc::Sender<int_msg::IntMsg>,
    pub periodic_check_interval: Duration,
}

impl SubTaskCheckStateBoot {
    pub async fn run(self) -> Result<(), Error> {
        loop {
            self.output
                .send(int_msg::IntMsg::PeriodicCheck)
                .await
                .unwrap();
            sleep(self.periodic_check_interval).await;
        }
    }
}
