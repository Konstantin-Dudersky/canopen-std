use tokio::sync::mpsc;

use crate::Error;

use super::int_msg::IntMsg;

pub struct TaskCanframe {
    pub input: mpsc::Receiver<Vec<u8>>,
    pub output: mpsc::Sender<IntMsg>,
}

impl TaskCanframe {
    pub async fn run(mut self) -> Result<(), Error> {
        while let Some(data) = self.input.recv().await {
            self.output.send(IntMsg::CanFrame(data)).await.unwrap()
        }

        Err(Error::TaskEnded {
            description: "RpdoConsumer/TaskCanframe",
        })
    }
}
