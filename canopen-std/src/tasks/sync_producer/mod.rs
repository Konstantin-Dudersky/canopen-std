use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{mpsc, Mutex},
    time::sleep,
};

use crate::{
    core::{
        object_dictionary::ObjectDictionary,
        sync::{encode_counter, SyncCounter},
    },
    Error, MsgOut,
};

pub struct TaskSyncProducer {
    pub output: mpsc::Sender<MsgOut>,
    pub od: Arc<Mutex<ObjectDictionary>>,
}

impl TaskSyncProducer {
    pub async fn run(self) -> Result<(), Error> {
        let obj_0x1005 = self.od.lock().await.read_sync_message_cob_id()?;
        // Выходим, если генерация SYNC сообщений отключена
        if !obj_0x1005.device_generate_sync {
            return Ok(());
        }
        let cob_id_sync_message = obj_0x1005.cob_id;

        let obj_0x1006 = self.od.lock().await.read_0x1006()?;
        let period = obj_0x1006.period;
        // Выходим, если период равен 0
        if period == Duration::from_millis(0) {
            return Ok(());
        }

        let obj_0x1019 = self.od.lock().await.read_0x1019()?;
        let counter_overflow = obj_0x1019.counter;

        let mut counter = SyncCounter::new(counter_overflow)?;

        loop {
            let can_frame = match counter.advance() {
                Some(count) => encode_counter(count).to_vec(),
                None => vec![],
            };
            let msg_out = MsgOut::CanFrame {
                id: cob_id_sync_message,
                data: can_frame,
            };

            self.output.send(msg_out).await.unwrap();

            sleep(period).await;
        }
    }
}

// pub fn produce_sync(&mut self) -> Option<TxFrame> {
//     let counter = self.sync_producer.as_mut()?;
//     Some(match counter.advance() {
//         Some(count) => TxFrame::new(sync::SYNC_COB_ID, &sync::encode_counter(count)),
//         None => TxFrame::new(sync::SYNC_COB_ID, &[]),
//     })
// }
