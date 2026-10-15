use tokio::sync::{broadcast, mpsc};

use crate::{Error, MsgOut};

pub struct TaskSyncConsumer {
    pub input: mpsc::Receiver<Option<u8>>,
    pub output: mpsc::Sender<MsgOut>,
    pub output_sync: broadcast::Sender<Option<u8>>,
}

impl TaskSyncConsumer {
    pub async fn run(mut self) -> Result<(), Error> {
        while let Some(counter) = self.input.recv().await {
            let msg_out = MsgOut::SyncReceived { counter };
            self.output.send(msg_out).await.unwrap();

            self.output_sync.send(counter).unwrap();
        }

        Ok(())
    }
}
