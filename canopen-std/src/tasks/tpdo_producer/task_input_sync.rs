use tokio::sync::{broadcast, mpsc};

use crate::Error;

use super::int_msg::IntMsg;

pub struct TaskInputSync {
    pub input: broadcast::Receiver<Option<u8>>,
    pub output: mpsc::Sender<IntMsg>,
}

impl TaskInputSync {
    pub async fn run(mut self) -> Result<(), Error> {
        while let Ok(counter) = self.input.recv().await {
            self.output.send(IntMsg::Sync(counter)).await.unwrap()
        }

        Err(Error::TaskEnded {
            description: "TpdoProducer/TaskInputSync",
        })
    }
}
