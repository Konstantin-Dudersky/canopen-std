use std::{collections::HashMap, sync::Arc, time::Duration};

use canopen_rs::NodeId;
use embedded_can::Id;
use tokio::{
    sync::{Mutex, broadcast, mpsc},
    task::JoinSet,
    time::sleep,
};
use tracing::error;

use crate::{
    Error, MsgIn, MsgOut,
    core::nmt::{NmtCommand, NmtStateMachine},
    eds::Eds,
    objects::ObjectDictionary,
    tasks::{self, StartupInfo},
    types::{Object, Value},
};

pub struct CANopenNode {
    /// Канал входящих сообщений
    pub input: mpsc::Receiver<MsgIn>,

    /// Канал исходящих сообщений
    pub output: mpsc::Sender<MsgOut>,

    /// CAN ID узла
    pub node_id: u8,

    /// Содержимое EDS-файла
    pub eds_file: &'static str,

    pub objects: Vec<Object>,
}

impl CANopenNode {
    pub async fn run(mut self) {
        let node_id = NodeId::new(self.node_id).unwrap();

        let nmt_state = NmtStateMachine::new();
        let nmt_state = Arc::new(Mutex::new(nmt_state));

        let mut od = Eds::parse(self.eds_file, node_id)
            .unwrap()
            .object_dictionary(node_id)
            .unwrap();
        self.add_entries(&mut od).unwrap();
        let od = Arc::new(Mutex::new(od));

        let startup_info = tasks::StartupInfo::new();

        let (tx_sdo_server, rx_sdo_server) = mpsc::channel::<(Id, Vec<u8>)>(100);
        let (tx_nmt_slave, rx_nmt_slave) = mpsc::channel::<Vec<u8>>(100);
        let (tx_heartbeat_consumer, rx_heartbeat_consumer) = mpsc::channel::<Id>(100);
        let (tx_heartbeat_to_nmt_slave, rx_heartbeat_to_nmt_slave) =
            mpsc::channel::<NmtCommand>(100);
        let (tx_nmt_master, rx_nmt_master) = mpsc::channel::<(u8, NmtCommand)>(100);

        let (tx_sync_consumer, rx_sync_consumer) = mpsc::channel::<Option<u8>>(100);

        let (tx_sync_broadcast, rx_sync_broadcast) = broadcast::channel::<Option<u8>>(100);

        let (tx_od_change, rx_od_change) = mpsc::channel::<(u16, u8, Value)>(100);

        let (tx_od_changed, rx_od_changed) = broadcast::channel::<()>(100);

        let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

        let task = tasks::TaskSdoServer {
            input: rx_sdo_server,
            output: self.output.clone(),
            od: od.clone(),
            nmt_state: nmt_state.clone(),
            startup_info: startup_info.clone(),
        };
        task_set.spawn(task.run());

        let task = tasks::TaskNmtSlave {
            input_can_frame: rx_nmt_slave,
            input_heartbeat_consumer: rx_heartbeat_to_nmt_slave,
            output: self.output.clone(),
            nmt_state: nmt_state.clone(),
            node_id,
        };
        task_set.spawn(task.run());

        let task = tasks::TaskHeartbeatConsumer {
            input: rx_heartbeat_consumer,
            output_nmt_slave: tx_heartbeat_to_nmt_slave,
            od: od.clone(),
        };
        task_set.spawn(task.run());

        let task = tasks::TaskHeartbeatProducer {
            output: self.output.clone(),
            od: od.clone(),
            nmt_state: nmt_state.clone(),
            node_id,
            startup_info: startup_info.clone(),
        };
        task_set.spawn(task.run());

        let task = tasks::TaskNmtMaster {
            input: rx_nmt_master,
            output: self.output.clone(),
        };
        task_set.spawn(task.run());

        let task = tasks::TaskSyncProducer {
            output: self.output.clone(),
            od: od.clone(),
        };
        task_set.spawn(task.run());

        let task = tasks::TaskSyncConsumer {
            input: rx_sync_consumer,
            output: self.output.clone(),
            output_sync: tx_sync_broadcast,
        };
        task_set.spawn(task.run());

        let task = tasks::TaskOdChange {
            input: rx_od_change,
            output: tx_od_changed,
            od: od.clone(),
        };
        task_set.spawn(task.run());

        spawn_tpdo_producer(
            &mut task_set,
            rx_sync_broadcast.resubscribe(),
            rx_od_changed,
            self.output.clone(),
            od.clone(),
            startup_info.clone(),
        )
        .await
        .unwrap();

        let rpdo_consumer = spawn_rpdo_consumer(
            &mut task_set,
            rx_sync_broadcast.resubscribe(),
            self.output.clone(),
            od.clone(),
            startup_info.clone(),
        )
        .await
        .unwrap();

        task_set.spawn(async move {
            sleep(Duration::from_millis(100)).await;
            let output = startup_info.lock().await.output();
            println!("{}", output);
            Ok(())
        });

        let task = tasks::Input {
            input: self.input,
            output_sdo_server: tx_sdo_server,
            output_nmt_slave: tx_nmt_slave,
            output_heartbeat_consumer: tx_heartbeat_consumer,
            output_nmt_master: tx_nmt_master,
            output_sync_consumer: tx_sync_consumer,
            output_od_change: tx_od_change,
            outputs_rpdo_consumer: rpdo_consumer,
            od: od.clone(),
        };
        task_set.spawn(task.run());

        while let Some(result) = task_set.join_next().await {
            let err = match result {
                Ok(Ok(())) => continue,
                Ok(Err(e)) => e.to_string(),
                Err(e) => e.to_string(),
            };
            error!("{:?}", err);
            break;
        }

        task_set.shutdown().await;
    }

    fn add_entries(&mut self, od: &mut ObjectDictionary) -> Result<(), Error> {
        for entry in self.objects.iter() {
            od.insert_object(entry.clone());
        }
        Ok(())
    }
}

async fn spawn_tpdo_producer(
    task_set: &mut JoinSet<Result<(), Error>>,
    mut rx_sync_broadcast: broadcast::Receiver<Option<u8>>,
    mut rx_od_changed: broadcast::Receiver<()>,
    output: mpsc::Sender<MsgOut>,
    od: Arc<Mutex<ObjectDictionary>>,
    startup_info: Arc<Mutex<StartupInfo>>,
) -> Result<(), Error> {
    let mut count = 0;

    for tpdo_index in 0..4 {
        let pdo_exist = od
            .lock()
            .await
            .read_tpdo_communication(tpdo_index)?
            .pdo_exist;
        if !pdo_exist {
            break;
        }

        count += 1;

        let task = tasks::TaskTpdoProducer {
            input_sync: rx_sync_broadcast.resubscribe(),
            input_od_changed: rx_od_changed.resubscribe(),
            output: output.clone(),
            od: od.clone(),
            tpdo_index,
            startup_info: startup_info.clone(),
        };
        task_set.spawn(task.run());
    }

    // Пустые потребители
    if count == 0 {
        task_set.spawn(async move {
            while rx_sync_broadcast.recv().await.is_ok() {}
            Ok(())
        });
        task_set.spawn(async move {
            while rx_od_changed.recv().await.is_ok() {}
            Ok(())
        });
    }

    Ok(())
}

async fn spawn_rpdo_consumer(
    task_set: &mut JoinSet<Result<(), Error>>,
    mut rx_sync_broadcast: broadcast::Receiver<Option<u8>>,
    output: mpsc::Sender<MsgOut>,
    od: Arc<Mutex<ObjectDictionary>>,
    startup_info: Arc<Mutex<StartupInfo>>,
) -> Result<HashMap<Id, mpsc::Sender<Vec<u8>>>, Error> {
    let mut rpdos: HashMap<Id, mpsc::Sender<Vec<u8>>> = HashMap::new();

    // Находим все активные RPDO
    for od_index in 0..=3 {
        let param = od.lock().await.read_rpdo_communication(od_index)?;

        if !param.pdo_exist {
            continue;
        }

        let (tx, rx) = mpsc::channel::<Vec<u8>>(100);
        rpdos.insert(param.cob_id, tx);

        let task = tasks::TaskRpdoConsumer {
            input_frame: rx,
            input_sync: rx_sync_broadcast.resubscribe(),
            output: output.clone(),
            od: od.clone(),
            startup_info: startup_info.clone(),
            sequence_number: od_index,
        };
        task_set.spawn(task.run());
    }

    // Пустые потребители
    if rpdos.is_empty() {
        task_set.spawn(async move {
            while rx_sync_broadcast.recv().await.is_ok() {}
            Ok(())
        });
    }

    Ok(rpdos)
}
