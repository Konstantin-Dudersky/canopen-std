use tokio::sync::{broadcast, mpsc};

use crate::Error;

use super::int_msg::IntMsg;

pub struct TaskInputOdChanged {
    pub input: broadcast::Receiver<()>,
    pub output: mpsc::Sender<IntMsg>,
}

impl TaskInputOdChanged {
    pub async fn run(mut self) -> Result<(), Error> {
        while self.input.recv().await.is_ok() {
            self.output.send(IntMsg::OdChanged).await.unwrap();
        }
        Err(Error::TaskEnd("TpdoProducer/TaskInputOdChanged"))
    }
}
