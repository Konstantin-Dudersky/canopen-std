use std::time::Duration;

use canopen_std::*;
use embedded_can::{Id, StandardId};
use tokio::sync::mpsc;

mod common;

use common::{init_logging, recv_until};

#[tokio::test]
async fn sync_producer() {
    init_logging();

    let node_id = 0x17;

    let (_tx_to_canopen, rx_to_canopen) = mpsc::channel::<MsgIn>(100);

    let (tx_from_canopen, mut rx_from_canopen) = mpsc::channel::<MsgOut>(100);

    let node = canopen_std::CANopenNode {
        input: rx_to_canopen,
        output: tx_from_canopen,
        node_id,
        eds_file: "",
        objects: vec![
            objects::SyncMessageCobId {
                cob_id: objects::SyncMessageCobId::COB_ID_DEFAULT,
                device_generate_sync: true,
            }
            .into(),
            objects::CommCyclePeriod {
                period: Duration::from_millis(10),
            }
            .into(),
            objects::SyncCounterOverflow { counter: 240 }.into(),
        ],
    };
    let node_handle = tokio::spawn(node.run());

    let msg = recv_until(&mut rx_from_canopen, Duration::from_secs(1), |m| {
        let (id, data) = match m {
            MsgOut::CanFrame { id, data, .. } => (id, data),
            _ => return false,
        };

        // Проверяем COB-ID
        if *id != Id::Standard(StandardId::new(0x080).expect("Invalid COB-ID for SYNC message")) {
            return false;
        }

        // Длина посылки должна быть 1 байт
        if data.len() != 1 {
            return false;
        }

        true
    })
    .await;

    assert!(msg.is_ok());

    node_handle.abort();
}
