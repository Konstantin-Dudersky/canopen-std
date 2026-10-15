use tokio::sync::{broadcast, mpsc};

use crate::Error;

use super::int_msg::IntMsg;

pub struct TaskSync {
    pub input: broadcast::Receiver<Option<u8>>,
    pub output: mpsc::Sender<IntMsg>,
}

impl TaskSync {
    pub async fn run(mut self) -> Result<(), Error> {
        while self.input.recv().await.is_ok() {
            self.output.send(IntMsg::Sync).await.unwrap()
        }

        Err(Error::TaskEnded {
            description: "RpdoConsumer/TaskSync",
        })
    }
}
