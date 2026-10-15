//! RPDO с адресом 0x217

mod common;

use std::time::Duration;

use canopen_std::{
    objects::{self, ObjectTrait, PdoMappingItem, RpdoCommunication, RpdoTransmissionType},
    AccessType, Address, CANopenNode, Entry, MsgIn, MsgOut, Value,
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
        eds: "",
        entries: [
            objects::RpdoCommunication {
                sequence_number: 0,
                pdo_exist: true,
                cob_id: RpdoCommunication::default_cob_id(0x17, 0).unwrap(),
                transmission_type: RpdoTransmissionType::EventDrivenManufacturer,
                inhibit_time: Duration::from_millis(0),
                event_timer: Duration::from_millis(0),
            }
            .entries(),
            objects::RpdoMapping {
                sequence_number: 0,
                items: vec![
                    PdoMappingItem {
                        index: 0x2000,
                        subindex: 1,
                        length: 8,
                    },
                    PdoMappingItem {
                        index: 0x2000,
                        subindex: 2,
                        length: 8,
                    },
                ],
            }
            .entries(),
            vec![
                Entry {
                    address: Address::new(0x2000, 1),
                    value: Value::Unsigned8(0),
                    access: AccessType::Ro,
                },
                Entry {
                    address: Address::new(0x2000, 2),
                    value: Value::Unsigned8(0),
                    access: AccessType::Ro,
                },
            ],
        ]
        .concat(),
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
