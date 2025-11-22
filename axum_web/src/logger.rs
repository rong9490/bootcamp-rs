use tracing_subscriber::EnvFilter;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::layer::SubscriberExt;

pub fn init() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_err| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer()
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_target(false)
        ).init();
}