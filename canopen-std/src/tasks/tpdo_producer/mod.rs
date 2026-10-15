mod int_msg;
mod task_input_od_changed;
mod task_input_sync;
mod task_main;
mod timer;

use std::{collections::HashMap, sync::Arc};

use tokio::{
    sync::{broadcast, mpsc, Mutex, Notify},
    task::JoinSet,
};
use tracing::error;

use crate::{core::object_dictionary::ObjectDictionary, Error, MsgOut};

use super::StartupInfo;

pub struct TaskTpdoProducer {
    pub input_sync: broadcast::Receiver<Option<u8>>,
    pub input_od_changed: broadcast::Receiver<()>,
    pub output: mpsc::Sender<MsgOut>,
    pub od: Arc<Mutex<ObjectDictionary>>,
    pub startup_info: Arc<Mutex<StartupInfo>>,
    pub tpdo_index: u16,
}

impl TaskTpdoProducer {
    pub async fn run(self) -> Result<(), Error> {
        let mut info: HashMap<String, String> = HashMap::new();

        let config = self.od.lock().await.read_0x1800(self.tpdo_index)?;
        info.insert("COB-ID".into(), format!("{:x?} (0x)", config.cob_id));

        let mapping = self.od.lock().await.read_0x1A00(self.tpdo_index)?;

        let (tx, rx) = mpsc::channel::<int_msg::IntMsg>(100);

        let timer_reset = Arc::new(Notify::new());

        let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

        // Принимаем SYNC
        let task = task_input_sync::TaskInputSync {
            input: self.input_sync,
            output: tx.clone(),
        };
        task_set.spawn(task.run());

        // Таймер Event Timer
        let task = timer::Timer {
            output: tx.clone(),
            interval: config.event_timer,
            value: int_msg::IntMsg::EventTimer,
            timer_reset: timer_reset.clone(),
        };
        task_set.spawn(task.run());

        // Таймер Inhibit Timer
        let task = timer::Timer {
            output: tx.clone(),
            value: int_msg::IntMsg::InhibitTimer,
            interval: config.inhibit_time,
            timer_reset: timer_reset.clone(),
        };
        task_set.spawn(task.run());

        // Ожидание изменений в словаре
        let task = task_input_od_changed::TaskInputOdChanged {
            input: self.input_od_changed.resubscribe(),
            output: tx.clone(),
        };
        task_set.spawn(task.run());

        // Основная логика
        let task = task_main::TaskMain {
            input: rx,
            output: self.output,
            od: self.od.clone(),
            config,
            mapping,
            timer_reset: timer_reset.clone(),
        };
        task_set.spawn(task.run());

        let task_name = format!("TPDO #{}", self.tpdo_index);
        self.startup_info
            .lock()
            .await
            .add_task_info(&task_name, info);

        while let Some(res) = task_set.join_next().await {
            error!("Result: {:?}", res);
        }

        Err(Error::TaskEnd("TaskTpdoProducer"))
    }
}
