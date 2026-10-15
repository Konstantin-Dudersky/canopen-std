use canopen_std::*;
use embedded_can::Frame;
use futures_util::StreamExt;
use socketcan::{tokio::CanSocket, CanFrame};
use tokio::sync::mpsc;
use tracing::info;

pub async fn receive_can_frames(tx: mpsc::Sender<MsgIn>) {
    let mut sock_rx = CanSocket::open("vcan0").unwrap();

    while let Some(Ok(frame)) = sock_rx.next().await {
        if matches!(frame, CanFrame::Data(_)) {
            tx.send(MsgIn::CanFrame {
                id: frame.id(),
                data: frame.data().to_vec(),
            })
            .await
            .unwrap();
        }
    }
}

pub async fn send_can_frames(mut rx: mpsc::Receiver<MsgOut>) {
    let sock_tx = CanSocket::open("vcan0").unwrap();

    while let Some(msg_out) = rx.recv().await {
        // info!("Send frame: {:x?}", msg_out);
        match msg_out {
            MsgOut::CanFrame { id, data } => {
                sock_tx
                    .write_frame(CanFrame::Data(Frame::new(id, &data).unwrap()))
                    .await
                    .unwrap();
            }
            MsgOut::NmtStateChanged(ns) => {
                info!("NMT state changed: {:?}", ns);
            }
            MsgOut::SyncReceived { counter: _ } => {}
            MsgOut::ObjectChanged {
                index: _,
                subindex: _,
                value: _,
            } => {}
        }
    }
}
