use std::time::Duration;

use canopen_std::{objects::ProducerHeartbeatTime, *};
use embedded_can::Id;
use tokio::sync::mpsc;

mod common;

use common::recv_until;

#[tokio::test]
async fn test() {
    let node_id = 0x17;

    let (_tx_to_canopen, rx_to_canopen) = mpsc::channel::<MsgIn>(100);

    let (tx_from_canopen, mut rx_from_canopen) = mpsc::channel::<MsgOut>(100);

    let node = canopen_std::CANopenNode {
        input: rx_to_canopen,
        output: tx_from_canopen,
        node_id,
        eds: "",
        entries: [ProducerHeartbeatTime {
            heartbeat_time: Duration::from_millis(10),
            heartbeat_time_access: AccessType::Ro,
        }
        .entries()]
        .concat(),
    };
    let node_handle = tokio::spawn(node.run());

    let msg = recv_until(&mut rx_from_canopen, Duration::from_secs(1), |m| {
        let (id, data) = match m {
            MsgOut::CanFrame { id, data, .. } => (id, data),
            _ => return false,
        };

        // Берём только стандартные идентификаторы
        let id = match id {
            Id::Standard(id) => id.as_raw(),
            Id::Extended(_) => return false,
        };

        // Проверяем COB-ID
        if id != 0x700 + node_id as u16 {
            return false;
        }

        // Длина посылки должна быть 1 байт
        if data.len() != 1 {
            return false;
        }

        // Пропускаем boot сообщение
        if data[0] == 0x00 {
            return false;
        }

        true
    })
    .await;

    assert!(msg.is_ok());

    node_handle.abort();
}
