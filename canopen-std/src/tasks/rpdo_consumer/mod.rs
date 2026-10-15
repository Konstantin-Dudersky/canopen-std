mod decode_rpdo;
mod int_msg;
mod task_canframe;
mod task_main;
mod task_sync;

use std::{collections::HashMap, sync::Arc};

use tokio::{
    sync::{broadcast, mpsc, Mutex},
    task::JoinSet,
};

use crate::{core::object_dictionary::ObjectDictionary, Error, MsgOut, Value};

use super::{startup_info::id_to_string, StartupInfo};
use int_msg::IntMsg;

pub struct TaskRpdoConsumer {
    pub input_frame: mpsc::Receiver<Vec<u8>>,
    pub input_sync: broadcast::Receiver<Option<u8>>,
    pub output: mpsc::Sender<MsgOut>,
    pub od: Arc<Mutex<ObjectDictionary>>,
    pub startup_info: Arc<Mutex<StartupInfo>>,
    pub sequence_number: u16,
}

impl TaskRpdoConsumer {
    pub async fn run(mut self) -> Result<(), Error> {
        let mut info: HashMap<String, String> = HashMap::new();

        let config = self
            .od
            .lock()
            .await
            .read_rpdo_communication(self.sequence_number)?;
        info.insert("COB-ID".into(), id_to_string(config.cob_id));
        info.insert(
            "Transmission type".into(),
            config.transmission_type.to_string(),
        );

        let mapping = self
            .od
            .lock()
            .await
            .read_rpdo_mapping(self.sequence_number)?;

        let task_name = format!("RPDO №{}", self.sequence_number);
        self.startup_info
            .lock()
            .await
            .add_task_info(&task_name, info);

        let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

        let (tx, rx) = mpsc::channel::<IntMsg>(100);

        let task = task_sync::TaskSync {
            input: self.input_sync,
            output: tx.clone(),
        };
        task_set.spawn(task.run());

        let task = task_canframe::TaskCanframe {
            input: self.input_frame,
            output: tx,
        };
        task_set.spawn(task.run());

        let task = task_main::TaskMain {
            input: rx,
            output: self.output,
        };
        task_set.spawn(task.run());

        while let Some(result) = task_set.join_next().await {
            result.unwrap()?;
        }

        Err(Error::TaskEnded {
            description: "TaskRpdoConsumer",
        })
    }
}
