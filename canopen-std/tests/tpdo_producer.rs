use std::time::Duration;

use canopen_std::{
    types::{AccessType, EntryArray, ObjectArray, Value},
    *,
};
use embedded_can::{Id, StandardId};
use tokio::sync::mpsc;

mod common;

use common::{init_logging, recv_until};
use tracing::info;

#[tokio::test]
async fn tpdo_producer() {
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
            objects::TpdoCommunication {
                seq_number: 1,
                pdo_exist: true,
                cob_id: objects::PdoCommunicationCobId::RelativeSelfNodeId,
                transmission_type: objects::TpdoTransmissionType::EventDrivenManufacturer,
                inhibit_time: Duration::from_millis(10),
                event_timer: Duration::from_millis(500),
                sync_start_value: 0,
            }
            .into(),
            objects::TpdoMapping {
                seq_number: 1,
                items: vec![objects::PdoMappingItem {
                    index: 0x2000,
                    subindex: 1,
                    length: 8,
                }],
            }
            .into(),
            ObjectArray {
                index: 0x2000,
                name: "ObjectArray".to_string(),
                entries: vec![EntryArray {
                    subindex: 1,
                    description: "ObjectArray".to_string(),
                    access_type: AccessType::Rw,
                    pdo_mapping: true,
                    value: Value::Unsigned8(78),
                }],
            }
            .into(),
        ],
    };
    let node_handle = tokio::spawn(node.run());

    let msg = recv_until(&mut rx_from_canopen, Duration::from_secs(1), |m| {
        let (id, data) = match m {
            MsgOut::CanFrame { id, data, .. } => (id, data),
            _ => return false,
        };

        info!("Received message: id={:?}, data={:?}", id, data);

        // Проверяем COB-ID
        if *id != Id::Standard(StandardId::new(0x297).expect("Invalid COB-ID for SYNC message")) {
            return false;
        }

        // Длина посылки должна быть 1 байт
        if data.len() != 1 {
            return false;
        }

        // Проверяем значение
        if data[0] != 78 {
            return false;
        }

        true
    })
    .await;

    assert!(msg.is_ok());

    node_handle.abort();
}
