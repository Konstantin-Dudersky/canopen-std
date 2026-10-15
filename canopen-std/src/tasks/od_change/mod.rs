use std::sync::Arc;

use tokio::sync::{broadcast, mpsc, Mutex};

use crate::{
    core::{
        datatypes::Value,
        object_dictionary::{Address, ObjectDictionary},
    },
    Error,
};

pub struct TaskOdChange {
    pub input: mpsc::Receiver<(u16, u8, Value)>,
    pub output: broadcast::Sender<()>,
    pub od: Arc<Mutex<ObjectDictionary>>,
}

impl TaskOdChange {
    pub async fn run(mut self) -> Result<(), Error> {
        while let Some((index, subindex, value)) = self.input.recv().await {
            let address = Address::new(index, subindex);

            {
                let mut od = self.od.lock().await;
                od.set(address, value)?;
            }

            self.output.send(()).unwrap();
        }

        Ok(())
    }
}
