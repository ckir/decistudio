use fern::Dispatch;
use log::LevelFilter;

/// Initialize logging for all server crates.
pub fn init_logging() {
    let _ = Dispatch::new()
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .apply();
}
