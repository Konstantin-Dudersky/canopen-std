mod task_input;
mod task_loop;

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use canopen_rs::NodeId;
use embedded_can::Id;
use tokio::{
    sync::{Mutex, mpsc},
    task::JoinSet,
};
use tracing::warn;

use crate::{core::nmt::NmtCommand, objects::ObjectDictionary};

pub struct TaskHeartbeatConsumer {
    pub input: mpsc::Receiver<Id>,
    pub output_nmt_slave: mpsc::Sender<NmtCommand>,
    pub od: Arc<Mutex<ObjectDictionary>>,
}

impl TaskHeartbeatConsumer {
    pub async fn run(self) -> Result<(), crate::Error> {
        let obj0x1016 = self.od.lock().await.read_consumer_heartbeat_time().unwrap();

        let error_behavior = self
            .od
            .lock()
            .await
            .read_error_behavior()
            .unwrap()
            .communication_error;

        let mut state = HeartbeatConsumerState::new();
        for item in obj0x1016.items {
            state.add_node(item.node_id, item.heartbeat_time);
        }
        let state = Arc::new(Mutex::new(state));

        let mut task_set = JoinSet::new();

        let task = task_input::TaskInput {
            input: self.input,
            heartbeat_consumer_state: state.clone(),
        };
        task_set.spawn(task.run());

        let task = task_loop::TaskLoop {
            error_behavior,
            output_nmt_slave: self.output_nmt_slave,
            heartbeat_consumer_state: state,
        };
        task_set.spawn(task.run());

        while let Some(result) = task_set.join_next().await {
            warn!("Result: {:?}", result);
        }

        Ok(())
    }
}

#[derive(Debug)]
struct HeartbeatConsumerStateItem {
    pub node_id: NodeId,
    pub last_seen: Instant,
    pub timeout: Duration,
}

#[derive(Debug, Default)]
pub struct HeartbeatConsumerState {
    nodes: Vec<HeartbeatConsumerStateItem>,
}

impl HeartbeatConsumerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node_id: NodeId, timeout: Duration) {
        self.nodes.push(HeartbeatConsumerStateItem {
            node_id,
            last_seen: Instant::now(),
            timeout,
        });
    }

    pub fn on_frame(&mut self, node_id: NodeId) {
        for node in &mut self.nodes {
            if node.node_id == node_id {
                node.last_seen = Instant::now();
                return;
            }
        }
    }

    /// Проверяем время с последнего Heartbeat
    pub fn timeout_occured(&mut self) -> bool {
        for node in &mut self.nodes {
            if node.last_seen.elapsed() > node.timeout {
                warn!("Node {:?} timed out", node.node_id);
                node.last_seen = Instant::now();
                return true;
            }
        }
        false
    }
}
