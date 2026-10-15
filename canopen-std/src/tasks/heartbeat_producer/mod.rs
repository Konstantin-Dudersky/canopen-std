use std::{collections::HashMap, sync::Arc, time::Duration};

use canopen_rs::NodeId;
use tokio::{
    sync::{mpsc, Mutex},
    time::sleep,
};

use crate::{
    core::{
        nmt::{encode_heartbeat, heartbeat_cob_id, NmtStateMachine},
        object_dictionary::ObjectDictionary,
    },
    Error, MsgOut,
};

use super::{startup_info::id_to_string, StartupInfo};

pub struct TaskHeartbeatProducer {
    pub output: mpsc::Sender<MsgOut>,
    pub od: Arc<Mutex<ObjectDictionary>>,
    pub nmt_state: Arc<Mutex<NmtStateMachine>>,
    pub node_id: NodeId,
    pub startup_info: Arc<Mutex<StartupInfo>>,
}

impl TaskHeartbeatProducer {
    pub async fn run(self) -> Result<(), Error> {
        let mut startup_info: HashMap<String, String> = HashMap::new();

        let heartbeat_time = self.od.lock().await.read_object_0x1017()?.heartbeat_time;

        // Если время hearbeat не задано, то завершаем задачу
        if heartbeat_time == Duration::from_millis(0) {
            startup_info.insert("Status".into(), "disabled".into());
            self.startup_info
                .lock()
                .await
                .add_task_info("Heartbeat producer", startup_info);
            return Ok(());
        }

        let cob_id_heartbeat = heartbeat_cob_id(self.node_id);

        startup_info.insert("Time".into(), format!("{:?}", heartbeat_time));
        startup_info.insert("COB-ID".into(), id_to_string(cob_id_heartbeat));
        self.startup_info
            .lock()
            .await
            .add_task_info("Heartbeat producer", startup_info);

        loop {
            let nmt_state = self.nmt_state.lock().await.state();
            let data = encode_heartbeat(nmt_state).to_vec();

            let msg_out = MsgOut::CanFrame {
                id: cob_id_heartbeat,
                data,
            };
            self.output.send(msg_out).await.unwrap();
            sleep(heartbeat_time).await;
        }
    }
}
