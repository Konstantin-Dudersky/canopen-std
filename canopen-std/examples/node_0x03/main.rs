use std::time::Duration;

use canopen_std::{
    types::{AccessType, Value},
    *,
};
use tokio::{sync::mpsc, task::JoinSet, time::sleep};
use tracing::warn;

#[path = "../common/mod.rs"]
mod common;

use common::{receive_can_frames, send_can_frames};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().pretty().init();

    let mut task_set = JoinSet::new();

    let (tx_to_canopen, rx_to_canopen) = mpsc::channel::<MsgIn>(100);

    let (tx_from_canopen, rx_from_canopen) = mpsc::channel::<MsgOut>(100);

    // Запускаем CANopen узел
    let node = CANopenNode {
        input: rx_to_canopen,
        output: tx_from_canopen,
        node_id: 0x03,
        eds: include_str!(
            "/home/konstantin/projects/plc-modules/firmware/ek_modules_eds_dbc/EDS/EK-IO-DQ-v0.2.0.eds"
        ),
        objects: vec![
            objects::ProducerHeartbeatTime {
                heartbeat_time: Duration::from_millis(500),
            }
            .into(),
        ],
    };

    task_set.spawn(node.run());

    // Запускаем получение CAN кадров
    task_set.spawn(receive_can_frames(tx_to_canopen.clone()));

    // Запускаем отправку CAN кадров
    task_set.spawn(send_can_frames(rx_from_canopen));

    // task_set.spawn(counter(tx_to_canopen.clone()));

    while let Some(result) = task_set.join_next().await {
        warn!("Result: {:?}", result);
    }
}

pub async fn counter(tx: mpsc::Sender<MsgIn>) {
    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(100)).await;
        tx.send(MsgIn::OdChange {
            index: 0x2000,
            subindex: 0,
            value: Value::Unsigned16(counter),
        })
        .await
        .unwrap();
        counter = counter.wrapping_add(1);
    }
}
