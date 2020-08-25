// mod utils;

extern crate aes;
extern crate block_modes;
extern crate hex_literal;
extern crate base64;

use wasm_bindgen::prelude::*;

use std::str;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-aes!");
}

#[wasm_bindgen]
pub fn zx_encrypt(data: String, key: String, iv: String) {
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let cipher = Aes128Cbc::new_var(key.as_bytes(), iv.as_bytes()).unwrap();

    // buffer must have enough space for message+padding
    let mut buffer = [0u8; 32];
    // copy message to the buffer
    let pos = data.len();
    buffer[..pos].copy_from_slice(data.as_bytes());
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();
    let result = base64::encode(&ciphertext);

    alert(&result);
}

#[wasm_bindgen]
pub fn zx_decrypt(data: String, key: String, iv: String) {
    let bytes = base64::decode(&data).unwrap();

    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let cipher = Aes128Cbc::new_var(key.as_bytes(), iv.as_bytes()).unwrap();

    let mut buffer = bytes.to_vec();
    let ciphertext = cipher.decrypt(&mut buffer).unwrap();
    let result = str::from_utf8(&ciphertext).unwrap();

    alert(&result);
}