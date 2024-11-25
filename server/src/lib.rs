/// Server configuration and cli options.
pub mod config;
/// When managing the server encounters errors.
pub(crate) mod error;

use once_cell::sync::OnceCell;
use std::time::Instant;

/// A globally accessible instant of the server's start time.
///
/// This should be set immediately on startup.
pub static START_TIME: OnceCell<Instant> = OnceCell::new();

pub fn get_version(product: &str) -> String {
    format!(
        "inferno {}.{}.{}",
        if cfg!(debug_assertions) {
            format!("dev-{}", &env!("GIT_HASH")[0..9])
        } else {
            env!("CARGO_PKG_VERSION").to_string()
        },
        product,
        &env!("GIT_DATE")[0..10]
    )
}
