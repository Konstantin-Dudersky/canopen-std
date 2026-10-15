use tracing_subscriber;

pub fn init_logging() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init();
}
