mod init_logging;

pub use init_logging::init_logging;

use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::timeout;

use canopen_std::*;

/// Ждёт сообщение, удовлетворяющее предикату, не дольше `max`.
/// Прочие сообщения отбрасываются.
pub async fn recv_until<F>(
    rx: &mut mpsc::Receiver<MsgOut>,
    max: Duration,
    mut pred: F,
) -> anyhow::Result<()>
where
    F: FnMut(&MsgOut) -> bool,
{
    let fut = async {
        loop {
            match rx.recv().await {
                Some(msg) if pred(&msg) => break msg,
                Some(_) => continue, // не тот тип — пропускаем
                None => panic!("output channel closed before match"),
            }
        }
    };

    timeout(max, fut).await?;
    Ok(())
}
