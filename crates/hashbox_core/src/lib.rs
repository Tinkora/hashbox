//! hashbox_core — multi-algorithm hashing toolbox.
//!
//! Pure Rust core with no I/O.  All algorithms are exposed through a uniform
//! API: one-shot [`hash`](hash::hash), streaming [`hash_streaming`],
//! and [`hmac_sha256`](hash::hmac_sha256) / [`hmac_sha512`](hash::hmac_sha512).
//!
//! On `wasm32` targets the `wasm` module is compiled and exposes
//! `#[wasm_bindgen]` functions for JavaScript.

mod error;
pub mod hash;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use error::CoreError;
pub use hash::{hash, hash_streaming, hmac_sha256, hmac_sha512, HashAlgorithm, HashWriter};
