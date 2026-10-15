use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{mpsc, Notify},
    time::sleep,
};
use tracing::trace;

use crate::Error;

use super::int_msg::IntMsg;

pub struct Timer<T>
where
    T: From<IntMsg>,
{
    pub output: mpsc::Sender<T>,
    pub value: T,
    pub interval: Duration,
    pub timer_reset: Arc<Notify>,
}

impl<T> Timer<T>
where
    T: From<IntMsg> + Copy + std::fmt::Debug,
{
    pub async fn run(self) -> Result<(), Error> {
        loop {
            trace!("Timer {:?} started", self.value);

            tokio::select! {
                // Таймер истёк — отправляем PDO принудительно
                _ = sleep(self.interval) => {
                    self.output.send(self.value).await.unwrap();
                    trace!("Timer {:?} elapsed", self.value);
                    self.timer_reset.notified().await;

                }
                // Событие отправило PDO — перезапускаем таймер
                _ = self.timer_reset.notified() => {
                    trace!("Timer {:?} reset", self.value);

                    // Ничего не делаем, loop создаст новый sleep
                }
            }
        }
    }
}
