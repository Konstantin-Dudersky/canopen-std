use tokio::sync::mpsc;

use crate::Error;

use super::int_msg::IntMsg;

pub struct TaskInput {
    pub input: mpsc::Receiver<Vec<u8>>,
    pub output: mpsc::Sender<IntMsg>,
}

impl TaskInput {
    pub async fn run(mut self) -> Result<(), Error> {
        while let Some(data) = self.input.recv().await {
            let out = IntMsg::CanFrameReceived(data);
            self.output.send(out).await.unwrap();
        }

        Ok(())
    }
}
