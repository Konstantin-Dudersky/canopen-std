use std::{collections::HashMap, sync::Arc};

use embedded_can::{Id, StandardId};
use tokio::sync::{Mutex, mpsc};

use crate::{
    Error, MsgIn,
    core::{
        nmt::{HEARTBEAT_COB_BASE, NMT_COMMAND_COB_ID, NmtCommand},
        sync::{self, SYNC_COB_ID},
    },
    objects::ObjectDictionary,
    types::Value,
};

pub struct Input {
    pub input: mpsc::Receiver<MsgIn>,
    pub output_sdo_server: mpsc::Sender<(Id, Vec<u8>)>,
    pub output_nmt_slave: mpsc::Sender<Vec<u8>>,
    pub output_heartbeat_consumer: mpsc::Sender<Id>,
    pub output_nmt_master: mpsc::Sender<(u8, NmtCommand)>,
    pub output_sync_consumer: mpsc::Sender<Option<u8>>,
    pub output_od_change: mpsc::Sender<(u16, u8, Value)>,
    pub outputs_rpdo_consumer: HashMap<Id, mpsc::Sender<Vec<u8>>>,
    pub od: Arc<Mutex<ObjectDictionary>>,
}

impl Input {
    pub async fn run(mut self) -> Result<(), Error> {
        let cob_id_sdo_server = self
            .od
            .lock()
            .await
            .read_sdo_server()?
            .cob_id_client_to_server;

        let cob_id_nmt: Id = NMT_COMMAND_COB_ID;

        let cob_id_sync: Id = SYNC_COB_ID;

        let cob_id_heartbeat_min = StandardId::new(HEARTBEAT_COB_BASE + 0x01).unwrap().into();
        let cob_id_heartbeat_max = StandardId::new(HEARTBEAT_COB_BASE + 0x7F).unwrap().into();

        // Вектор идентификаторов RPDO
        let cob_ids_rpdo = self
            .outputs_rpdo_consumer
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        while let Some(msg_in) = self.input.recv().await {
            match msg_in {
                MsgIn::CanFrame { id, data } => {
                    if id == cob_id_nmt {
                        // NMT command from master
                        self.output_nmt_slave.send(data).await.unwrap();
                    } else if id == cob_id_sync {
                        // SYNC
                        let data = sync::decode(&data)?;
                        self.output_sync_consumer.send(data).await.unwrap();
                    } else if id == cob_id_sdo_server {
                        // SDO request from server
                        self.output_sdo_server.send((id, data)).await.unwrap();
                    } else if id >= cob_id_heartbeat_min && id <= cob_id_heartbeat_max {
                        // Heartbeat
                        self.output_heartbeat_consumer.send(id).await.unwrap();
                    } else if cob_ids_rpdo.contains(&id) {
                        // RPDO
                        for (cob_id_rpdo, tx) in &self.outputs_rpdo_consumer {
                            if id == *cob_id_rpdo {
                                tx.send(data).await.unwrap();
                                break;
                            }
                        }
                    }
                }
                MsgIn::NmtMasterCommand { target, command } => {
                    self.output_nmt_master
                        .send((target, command))
                        .await
                        .unwrap();
                }

                MsgIn::ObjectDictdChange {
                    index,
                    subindex,
                    value,
                } => self
                    .output_od_change
                    .send((index, subindex, value))
                    .await
                    .unwrap(),

                MsgIn::SdoRead { .. } => {}

                MsgIn::SdoWrite { .. } => {}
            }
        }

        Err(Error::TaskEnd("Input"))
    }
}
