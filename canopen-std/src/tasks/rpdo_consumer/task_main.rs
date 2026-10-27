use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};

use crate::{
    Error, MsgOut,
    objects::{ObjectDictionary, RpdoCommunication, RpdoMapping, RpdoTransmissionType},
};

use super::{decode_rpdo::decode_rpdo, int_msg::IntMsg};

pub struct TaskMain {
    pub input: mpsc::Receiver<IntMsg>,
    pub output: mpsc::Sender<MsgOut>,
    pub od: Arc<Mutex<ObjectDictionary>>,
    pub rpdo_config: RpdoCommunication,
    pub rpdo_mapping: RpdoMapping,
}

impl TaskMain {
    pub async fn run(mut self) -> Result<(), Error> {
        let mut rpdo_data: Vec<u8> = vec![];

        while let Some(int_msg) = self.input.recv().await {
            match int_msg {
                IntMsg::Sync => match self.rpdo_config.transmission_type {
                    RpdoTransmissionType::Syncronous => {
                        decode_and_send(&self.rpdo_mapping, &rpdo_data, &self.od, &self.output)
                            .await?;
                    }
                    _ => continue,
                },
                IntMsg::CanFrame(data) => {
                    rpdo_data = data;

                    match self.rpdo_config.transmission_type {
                        RpdoTransmissionType::EventDrivenManufacturer
                        | RpdoTransmissionType::EventDrivenProfile => {
                            decode_and_send(&self.rpdo_mapping, &rpdo_data, &self.od, &self.output)
                                .await?;
                        }
                        _ => continue,
                    }
                }
            }
        }

        Err(Error::TaskEnded {
            description: "RpdoConsumer/TaskMain",
        })
    }
}

async fn decode_and_send(
    rpdo_mapping: &RpdoMapping,
    rpdo_data: &[u8],
    od: &Arc<Mutex<ObjectDictionary>>,
    output: &mpsc::Sender<MsgOut>,
) -> Result<(), Error> {
    let mut od = od.lock().await;

    let values = decode_rpdo(rpdo_mapping, rpdo_data, &od).unwrap();

    for (index, subindex, value) in values.iter().cloned() {
        od.set(index, subindex, value)?;
    }

    drop(od);

    for (index, subindex, value) in values {
        let msg_out = MsgOut::ObjectChanged {
            index,
            subindex,
            value,
        };
        output.send(msg_out).await.unwrap();
    }

    Ok(())
}
