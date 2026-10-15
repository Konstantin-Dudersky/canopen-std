use canopen_rs::NodeId;
use tokio::sync::mpsc;

use crate::{
    core::nmt::{encode_command, NmtCommand, NMT_COMMAND_COB_ID},
    Error, MsgOut,
};

pub struct TaskNmtMaster {
    pub input: mpsc::Receiver<(u8, NmtCommand)>,
    pub output: mpsc::Sender<MsgOut>,
}

impl TaskNmtMaster {
    pub async fn run(mut self) -> Result<(), Error> {
        while let Some((node_id, command)) = self.input.recv().await {
            let node_id = match node_id {
                0 => NodeId::BROADCAST,
                _ => NodeId::new(node_id).unwrap(),
            };
            let data = encode_command(command, node_id).to_vec();

            let msg_out = MsgOut::CanFrame {
                id: NMT_COMMAND_COB_ID,
                data,
            };
            self.output.send(msg_out).await.unwrap();
        }

        Ok(())
    }
}
