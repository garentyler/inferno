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
