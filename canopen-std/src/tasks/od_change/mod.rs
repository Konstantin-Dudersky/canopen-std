use std::sync::Arc;

use tokio::sync::{broadcast, mpsc, Mutex};

use crate::{objects::ObjectDictionary, types::Value, Error};

pub struct TaskOdChange {
    pub input: mpsc::Receiver<(u16, u8, Value)>,
    pub output: broadcast::Sender<()>,
    pub od: Arc<Mutex<ObjectDictionary>>,
}

impl TaskOdChange {
    pub async fn run(mut self) -> Result<(), Error> {
        while let Some((index, subindex, value)) = self.input.recv().await {
            {
                let mut od = self.od.lock().await;
                od.set(index, subindex, value)?;
            }

            self.output.send(()).unwrap();
        }

        Ok(())
    }
}
