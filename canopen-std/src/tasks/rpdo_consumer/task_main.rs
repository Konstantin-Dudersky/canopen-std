use tokio::sync::mpsc;

use crate::{Error, MsgOut, types::Value};

use super::int_msg::IntMsg;

pub struct TaskMain {
    pub input: mpsc::Receiver<IntMsg>,
    pub output: mpsc::Sender<MsgOut>,
}

impl TaskMain {
    pub async fn run(mut self) -> Result<(), Error> {
        while let Some(int_msg) = self.input.recv().await {
            let msg_out = MsgOut::ObjectChanged {
                index: 0x2001,
                subindex: 1,
                value: Value::Unsigned8(0x18),
            };
            self.output.send(msg_out).await.unwrap();
        }

        Err(Error::TaskEnded {
            description: "RpdoConsumer/TaskMain",
        })
    }
}
