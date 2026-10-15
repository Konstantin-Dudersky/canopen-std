use std::time::Duration;

use canopen_std::*;
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
        node_id: 0x01,
        eds: include_str!("./EK-CP-LLP-v0.2.0.eds"),
        entries: vec![],
    };

    task_set.spawn(node.run());

    // Запускаем получение CAN кадров
    task_set.spawn(receive_can_frames(tx_to_canopen.clone()));

    // Запускаем отправку CAN кадров
    task_set.spawn(send_can_frames(rx_from_canopen));

    sleep(Duration::from_millis(300)).await;

    tx_to_canopen
        .send(MsgIn::NmtMasterCommand {
            target: 0x0,
            command: NmtCommand::StartRemoteNode,
        })
        .await
        .unwrap();

    while let Some(result) = task_set.join_next().await {
        warn!("Result: {:?}", result);
    }
}
