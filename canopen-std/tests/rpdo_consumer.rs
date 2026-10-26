//! RPDO с адресом 0x217

mod common;

use std::time::Duration;

use canopen_std::{
    CANopenNode, MsgIn, MsgOut, objects,
    types::{AccessType, EntryRecord, ObjectRecord, Value},
};
use common::{init_logging, recv_until};
use embedded_can::StandardId;
use tokio::sync::mpsc;

#[tokio::test]
async fn rpdo_consumer() {
    init_logging();

    let node_id = 0x17;

    let (tx_to_canopen, rx_to_canopen) = mpsc::channel::<MsgIn>(100);

    let (tx_from_canopen, mut rx_from_canopen) = mpsc::channel::<MsgOut>(100);

    let node = CANopenNode {
        input: rx_to_canopen,
        output: tx_from_canopen,
        node_id,
        eds_file: "",
        objects: vec![
            objects::RpdoCommunication {
                seq_number: 0,
                pdo_exist: true,
                cob_id: objects::PdoCommunicationCobId::RelativeSelfNodeId,
                transmission_type: objects::RpdoTransmissionType::EventDrivenManufacturer,
                inhibit_time: Duration::from_millis(0),
                event_timer: Duration::from_millis(0),
            }
            .into(),
            objects::RpdoMapping {
                seq_number: 0,
                items: vec![
                    objects::PdoMappingItem {
                        index: 0x2000,
                        subindex: 1,
                        length: 8,
                    },
                    objects::PdoMappingItem {
                        index: 0x2000,
                        subindex: 2,
                        length: 8,
                    },
                ],
            }
            .into(),
            ObjectRecord {
                index: 0x2000,
                name: "Test".into(),
                entries: vec![
                    EntryRecord {
                        subindex: 1,
                        description: "Test".into(),
                        access_type: AccessType::Ro,
                        pdo_mapping: false,
                        value: Value::Unsigned8(0),
                    },
                    EntryRecord {
                        subindex: 2,
                        description: "Test".into(),
                        access_type: AccessType::Ro,
                        pdo_mapping: false,
                        value: Value::Unsigned8(0),
                    },
                ],
            }
            .into(),
        ],
    };

    let node_handle = tokio::spawn(node.run());

    // Отправляем в CANopen полученное по шине RPDO
    tx_to_canopen
        .send(MsgIn::CanFrame {
            id: StandardId::new(0x217).unwrap().into(),
            data: vec![0x17, 0x18],
        })
        .await
        .unwrap();

    let msg = recv_until(&mut rx_from_canopen, Duration::from_secs(1), |m| {
        let (index, subindex, value) = match m {
            MsgOut::ObjectChanged {
                index,
                subindex,
                value,
            } => (*index, *subindex, *value),
            _ => return false,
        };

        if index != 0x2000 || subindex != 1 {
            return false;
        }

        if value != Value::Unsigned8(0x18) {
            return false;
        }

        true
    })
    .await;

    if let Err(err) = &msg {
        tracing::error!("Error: {err}");
    }

    assert!(msg.is_ok());

    node_handle.abort();
}
