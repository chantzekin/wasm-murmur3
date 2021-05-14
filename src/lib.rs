// src/lib.rs

extern crate wasm_bindgen;

use std::io::Cursor;
use wasm_bindgen::prelude::*;
use js_sys::Error as JsError;
use murmur3::murmur3_x64_128;

#[wasm_bindgen]
pub fn hex128(source: &str, seed :u32) -> Result<String, JsValue> {
    let hashed = match murmur3_x64_128(&mut Cursor::new(source), seed) {
        Ok(u128) => u128,
        Err(_) => return Err(
            JsError::new("InternalError: Execute murmur3::murmur3_x64_128 faild").into()
        ),
    };

    let bytes = hashed.to_be_bytes();
    let len = bytes.len();
    let half_len = len / 2;
    let out = format!(
        "{}{}",
        hex::encode(&bytes[half_len..len]),
        hex::encode(&bytes[0..half_len])
    );

    Ok(out)
}
