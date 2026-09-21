//! WASM bindings exposed to JavaScript.
//!
//! Every public function is annotated with `#[wasm_bindgen]` and catches
//! [`CoreError`] into a `JsValue` carrying stable `code` and `message`
//! fields.

use crate::error::CoreError;
use crate::hash::{self, HashAlgorithm};
use wasm_bindgen::prelude::*;

/// Maps a [`CoreError`] into a `JsValue` with `code` and `message` fields.
fn core_err(e: CoreError) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"code".into(), &e.code().into()).ok();
    js_sys::Reflect::set(&obj, &"message".into(), &e.to_string().into()).ok();
    obj.into()
}

/// Hash raw bytes with the given algorithm.
///
/// `algorithm` is the key string (`"sha256"`, `"blake3"`, …).
#[wasm_bindgen(js_name = wasmHash)]
pub fn wasm_hash(algorithm: &str, input: &[u8]) -> Result<String, JsValue> {
    let alg = HashAlgorithm::from_key(algorithm).map_err(core_err)?;
    Ok(hash::hash(alg, input))
}

/// Convenience: hash a UTF-8 text string.
///
/// This is equivalent to encoding the string as UTF-8 and calling
/// [`wasm_hash`].
#[wasm_bindgen(js_name = wasmHashText)]
pub fn wasm_hash_text(algorithm: &str, text: &str) -> Result<String, JsValue> {
    let alg = HashAlgorithm::from_key(algorithm).map_err(core_err)?;
    Ok(hash::hash(alg, text.as_bytes()))
}

/// Compute HMAC with the given algorithm (`"sha256"` or `"sha512"`) and
/// return a lowercase hex digest.
#[wasm_bindgen(js_name = wasmHmac)]
pub fn wasm_hmac(algorithm: &str, key: &[u8], message: &[u8]) -> Result<String, JsValue> {
    match algorithm.to_lowercase().as_str() {
        "sha256" | "sha-256" => hash::hmac_sha256(key, message).map_err(core_err),
        "sha512" | "sha-512" => hash::hmac_sha512(key, message).map_err(core_err),
        other => Err(core_err(CoreError::UnsupportedAlgorithm(other.to_owned()))),
    }
}

/// Return a JSON array of supported algorithm descriptors for use in JS UIs.
///
/// Each entry has `key`, `label`, and `outputLen` (bytes).
#[wasm_bindgen(js_name = wasmListAlgorithms)]
pub fn wasm_list_algorithms() -> JsValue {
    let list: Vec<JsValue> = HashAlgorithm::ALL
        .iter()
        .map(|a| {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"key".into(), &a.key().into()).ok();
            js_sys::Reflect::set(&obj, &"label".into(), &a.label().into()).ok();
            js_sys::Reflect::set(
                &obj,
                &"outputLen".into(),
                &(a.output_len() as u32).into(),
            )
            .ok();
            obj.into()
        })
        .collect();

    let arr = js_sys::Array::new();
    for item in &list {
        arr.push(item);
    }
    arr.into()
}
