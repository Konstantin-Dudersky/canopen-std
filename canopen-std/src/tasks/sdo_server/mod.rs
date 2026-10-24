use std::{collections::HashMap, sync::Arc};

use embedded_can::Id;
use tokio::sync::{mpsc, Mutex};
use tracing::warn;

use crate::{
    core::{nmt::NmtStateMachine, sdo::server::SdoServer},
    objects::ObjectDictionary,
    Error, MsgOut,
};

use super::{startup_info::id_to_string, StartupInfo};

pub struct TaskSdoServer {
    pub input: mpsc::Receiver<(Id, Vec<u8>)>,
    pub output: mpsc::Sender<MsgOut>,
    pub od: Arc<Mutex<ObjectDictionary>>,
    pub nmt_state: Arc<Mutex<NmtStateMachine>>,
    pub startup_info: Arc<Mutex<StartupInfo>>,
}

impl TaskSdoServer {
    pub async fn run(mut self) -> Result<(), Error> {
        let request_cob_id = self
            .od
            .lock()
            .await
            .read_sdo_server()?
            .cob_id_client_to_server;

        let response_cob_id = self
            .od
            .lock()
            .await
            .read_sdo_server()?
            .cob_id_server_to_client;

        let startup_info: HashMap<String, String> = HashMap::from_iter([
            ("Request COB-ID".into(), id_to_string(request_cob_id)),
            ("Response COB-ID".into(), id_to_string(response_cob_id)),
        ]);
        self.startup_info
            .lock()
            .await
            .add_task_info("SDO server", startup_info);

        let mut sdo_server = SdoServer::new(request_cob_id, response_cob_id);

        while let Some((id, data)) = self.input.recv().await {
            if !self.nmt_state.lock().await.is_sdo_server_enabled() {
                continue;
            }

            if id != request_cob_id {
                continue;
            }

            let response = {
                let mut od = self.od.lock().await;
                sdo_server.on_frame(&mut od, &data)
            };

            if let Some(response) = response {
                let msg_out = MsgOut::CanFrame {
                    id: sdo_server.response_cob_id(),
                    data: response.to_vec(),
                };
                self.output.send(msg_out).await.unwrap();
            } else {
                warn!("No response for SDO request");
            }
        }

        Err(Error::TaskEnded {
            description: "TaskSdoServer",
        })
    }
}
