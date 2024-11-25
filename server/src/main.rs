use tracing::{info, warn};
use tracing_subscriber::prelude::*;

#[tracing::instrument]
pub fn main() {
    inferno::START_TIME
        .set(std::time::Instant::now())
        .expect("could not set inferno::START_TIME");

    // Set up logging.
    let file_writer =
        tracing_appender::rolling::daily(&inferno::config::Args::instance().log_dir, "log");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_writer);

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            inferno::config::Args::instance()
                .log_level
                .unwrap_or(if cfg!(debug_assertions) {
                    tracing::Level::DEBUG
                } else {
                    tracing::Level::INFO
                }),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_ansi(false)
                .with_writer(file_writer),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_writer(std::io::stdout),
        )
        .init();

    // Load the config.
    let config = inferno::config::Config::load();

    info!("Starting {} on port {}", config.server_version, config.port);
    info!("This software is licensed under GPLv3.");
    // Start the server.
    info!(
        "Done! Start took {:?}",
        inferno::START_TIME.get().unwrap().elapsed()
    );
}
