use std::sync::Arc;

use bitvec::{order::Msb0, view::BitView};
use tokio::sync::{mpsc, Mutex, Notify};
use tracing::trace;

use crate::core::object_dictionary::objects_old::{
    TpdoCommParameters, TpdoMapping, TpdoTransmissionType,
};
use crate::{
    core::object_dictionary::{Address, ObjectDictionary},
    Error, MsgOut,
};

use super::int_msg::IntMsg;

pub struct TaskMain {
    pub input: mpsc::Receiver<IntMsg>,
    pub output: mpsc::Sender<MsgOut>,
    pub od: Arc<Mutex<ObjectDictionary>>,
    pub config: TpdoCommParameters,
    pub mapping: TpdoMapping,
    pub timer_reset: Arc<Notify>,
}

impl TaskMain {
    pub async fn run(mut self) -> Result<(), Error> {
        let mut data_prev: Vec<u8> = {
            let od = self.od.lock().await;
            pack_tpdo(&self.mapping, &od)?
        };
        let mut data_for_send: Option<Vec<u8>> = None;

        let mut state = match self.config.transmission_type {
            TpdoTransmissionType::SyncronousAcyclic => State::SyncAcyclicWaitEvent,
            TpdoTransmissionType::Syncronous(max_counter) => State::SyncWait {
                max_counter,
                current_counter: max_counter,
            },
            TpdoTransmissionType::EventDrivenManufacturer
            | TpdoTransmissionType::EventDrivenProfile => State::ProfileSend,
            TpdoTransmissionType::Reserved => todo!(),
        };

        while let Some(msg) = self.input.recv().await {
            match msg {
                IntMsg::Sync(sync_counter) => {
                    state = match state {
                        State::SyncAcyclicWaitEvent => {
                            if data_for_send.is_some() {
                                State::SyncAcyclicSend
                            } else {
                                State::SyncAcyclicWaitEvent
                            }
                        }

                        State::SyncAcyclicSendWaitSync => State::SyncAcyclicSend,

                        State::SyncWait {
                            max_counter,
                            current_counter,
                        } => {
                            let sync_counter_resetted = if let Some(sync_counter) = sync_counter {
                                sync_counter == 1
                            } else {
                                false
                            };
                            let current_counter = current_counter.saturating_sub(1);
                            if current_counter == 0 || sync_counter_resetted {
                                State::SyncSend { max_counter }
                            } else {
                                State::SyncWait {
                                    max_counter,
                                    current_counter,
                                }
                            }
                        }

                        _ => state,
                    }
                }

                IntMsg::OdChanged => {
                    let data = {
                        let od = self.od.lock().await;
                        pack_tpdo(&self.mapping, &od)?
                    };
                    if data != data_prev {
                        data_for_send = Some(data);
                    }
                }

                IntMsg::InhibitTimer => {
                    state = match state {
                        State::SyncAcyclicWaitInhibit => State::SyncAcyclicWaitEvent,
                        State::ProfileWaitInhibit => State::ProfileWaitEvent,
                        _ => state,
                    }
                }

                IntMsg::EventTimer => {
                    state = match state {
                        State::SyncAcyclicWaitEvent => State::SyncAcyclicSendWaitSync,
                        State::ProfileWaitEvent => State::ProfileSend,
                        _ => state,
                    }
                }
            }

            state = match state {
                State::SyncAcyclicSend => {
                    let data = match data_for_send.take() {
                        Some(data) => {
                            data_prev = data.clone();
                            data
                        }
                        None => data_prev.clone(),
                    };
                    let msg_out = MsgOut::CanFrame {
                        id: self.config.cob_id,
                        data,
                    };
                    self.output.send(msg_out).await.unwrap();

                    trace!("reset timers");
                    self.timer_reset.notify_waiters();

                    State::SyncAcyclicWaitInhibit
                }
                State::SyncAcyclicWaitInhibit => State::SyncAcyclicWaitInhibit,
                State::SyncAcyclicWaitEvent => {
                    if data_for_send.is_some() {
                        State::SyncAcyclicSend
                    } else {
                        State::SyncAcyclicWaitEvent
                    }
                }
                State::SyncAcyclicSendWaitSync => State::SyncAcyclicSendWaitSync,

                State::SyncSend { max_counter } => {
                    let data = match data_for_send.take() {
                        Some(data) => {
                            data_prev = data.clone();
                            data
                        }
                        None => data_prev.clone(),
                    };
                    let msg_out = MsgOut::CanFrame {
                        id: self.config.cob_id,
                        data,
                    };
                    self.output.send(msg_out).await.unwrap();

                    State::SyncWait {
                        max_counter,
                        current_counter: max_counter,
                    }
                }
                State::SyncWait {
                    max_counter,
                    current_counter,
                } => State::SyncWait {
                    max_counter,
                    current_counter,
                },

                State::ProfileSend => {
                    let data = match data_for_send.take() {
                        Some(data) => {
                            data_prev = data.clone();
                            data
                        }
                        None => data_prev.clone(),
                    };

                    let msg_out = MsgOut::CanFrame {
                        id: self.config.cob_id,
                        data,
                    };
                    self.output.send(msg_out).await.unwrap();

                    trace!("reset timers");
                    self.timer_reset.notify_waiters();

                    State::ProfileWaitInhibit
                }
                State::ProfileWaitInhibit => State::ProfileWaitInhibit,
                State::ProfileWaitEvent => {
                    if data_for_send.is_some() {
                        State::ProfileSend
                    } else {
                        State::ProfileWaitEvent
                    }
                }
            };
        }

        Err(Error::TaskEnd("TpdoProducer/Main"))
    }
}

fn pack_tpdo(mapping: &TpdoMapping, od: &ObjectDictionary) -> Result<Vec<u8>, Error> {
    let len_bits: usize = mapping.items.iter().map(|i| i.length as usize).sum();
    let len_bytes = len_bits.div_ceil(8);

    let mut data = vec![0u8; len_bytes];
    let bits = data.view_bits_mut::<Msb0>();
    let mut offset = 0;
    for item in &mapping.items {
        let width = item.length as usize;
        let addr = Address::new(item.index, item.subindex);
        let value = od.read(addr)?;
        if value.data_type().is_variable() || value.bit_size() != width {
            return Err(Error::TypeMismatch);
        }

        value.encode(bits, offset)?;
        offset += width;
    }
    Ok(data)
}

enum State {
    SyncAcyclicSend,
    SyncAcyclicWaitInhibit,
    SyncAcyclicWaitEvent,
    SyncAcyclicSendWaitSync,
    SyncSend {
        max_counter: u8,
    },
    SyncWait {
        max_counter: u8,
        current_counter: u8,
    },
    ProfileSend,
    ProfileWaitInhibit,
    ProfileWaitEvent,
}
