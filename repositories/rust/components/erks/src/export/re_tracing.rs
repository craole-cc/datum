pub use tracing::{debug, error, info, trace, warn};
// pub use tracing_subscriber;

pub fn set_tracing_level(lvl: tracing::Level) {
  tracing_subscriber::fmt()
    .with_max_level(lvl)
    .without_time()
    .with_target(false)
    .init();
}

pub fn set_tracing_trace() {
  set_tracing_level(tracing::Level::TRACE);
}

pub fn set_tracing_debug() {
  set_tracing_level(tracing::Level::DEBUG);
}

pub fn set_tracing_info() {
  set_tracing_level(tracing::Level::INFO);
}

pub fn set_tracing_warnings() {
  set_tracing_level(tracing::Level::WARN);
}

pub fn set_tracing_errors() {
  set_tracing_level(tracing::Level::ERROR);
}
