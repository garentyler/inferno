/// This type represents all possible errors that can occur when running the server.
#[allow(dead_code)]
#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum Error {}

/// Alias for a Result with the error type `inferno_server::error::Error`.
pub type Result<T> = std::result::Result<T, Error>;
