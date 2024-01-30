//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

use clique_crypto::aes::AES;

use k256::{elliptic_curve::sec1::ToEncodedPoint, SecretKey};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn with_ecdh() {
    let alice = [1u8; 32];
    let sk = SecretKey::from_slice(&alice).unwrap();
    let pk = sk.public_key();
    let aes = AES::with_ecdh(
        sk.to_bytes().as_slice(),
        pk.to_encoded_point(false).as_bytes(),
    );

    let data = aes.encrypt(b"hello world").unwrap();
    assert_eq!(b"hello world".to_vec(), aes.decrypt(&data).unwrap());
}

#[wasm_bindgen_test]
fn with_password() {
    let aes = AES::with_password("12345678");
    let data = aes.encrypt(b"hello world").unwrap();
    assert_eq!(b"hello world".to_vec(), aes.decrypt(&data).unwrap());
}
