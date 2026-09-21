//! Lightweight, zero-copy error type for the hashbox contract.
//!
//! Every variant carries a stable machine-readable [`code`](CoreError::code)
//! for Web, CLI, and Agent consumers.

use thiserror::Error;

/// Stable error type for the hashbox core crate.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    /// The requested algorithm name is not recognised.
    #[error("Unsupported hash algorithm: {0}")]
    UnsupportedAlgorithm(String),

    /// The HMAC key cannot be empty.
    #[error("HMAC key must not be empty")]
    HmacInvalidKey,
}

impl CoreError {
    /// Returns a stable, SCREAMING_SNAKE_CASE error code suitable for
    /// programmatic matching in JS, CLI, or Agent tooling.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UnsupportedAlgorithm(_) => "UNSUPPORTED_ALGORITHM",
            Self::HmacInvalidKey => "HMAC_INVALID_KEY",
        }
    }
}
