//! Thin re-export crate that exposes hashbox_core's WASM bindings
//! as a cdylib for wasm-pack / bundler consumption.
//!
//! All logic lives in `hashbox_core::wasm`; this crate only compiles
//! and links them into a `.wasm` binary.

pub use hashbox_core::wasm::*;
