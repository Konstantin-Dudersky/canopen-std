use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::{
    sync::{Mutex, mpsc},
    time::sleep,
};

use crate::{
    Error, MsgOut,
    core::sync::{SyncCounter, encode_counter},
    objects::ObjectDictionary,
    tasks::{StartupInfo, startup_info::id_to_string},
};

pub struct TaskSyncProducer {
    pub output: mpsc::Sender<MsgOut>,
    pub od: Arc<Mutex<ObjectDictionary>>,
    pub startup_info: Arc<Mutex<StartupInfo>>,
}

impl TaskSyncProducer {
    pub async fn run(self) -> Result<(), Error> {
        let mut startup_info: HashMap<String, String> = HashMap::new();

        let obj_0x1005 = self.od.lock().await.read_sync_message_cob_id()?;
        // Выходим, если генерация SYNC сообщений отключена
        if !obj_0x1005.device_generate_sync {
            startup_info.insert("Status".into(), "disabled".into());
            self.startup_info
                .lock()
                .await
                .add_task_info("SYNC producer", startup_info);
            return Ok(());
        }
        let cob_id_sync_message = obj_0x1005.cob_id;

        let obj_0x1006 = self.od.lock().await.read_comm_cycle_period()?;
        let period = obj_0x1006.period;
        // Выходим, если период равен 0
        if period == Duration::from_millis(0) {
            return Ok(());
        }

        let obj_0x1019 = self.od.lock().await.read_sync_counter_overflow()?;
        let counter_overflow = obj_0x1019.counter;

        startup_info.insert("Time".into(), format!("{:?}", period));
        startup_info.insert("COB-ID".into(), id_to_string(cob_id_sync_message));
        startup_info.insert("Counter".into(), counter_overflow.to_string());
        self.startup_info
            .lock()
            .await
            .add_task_info("SYNC producer", startup_info);

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
