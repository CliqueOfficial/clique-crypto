//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use std::vec;

use wasm_bindgen_test::*;

use clique_crypto::shamir::Shamir;

use k256::{elliptic_curve::sec1::ToEncodedPoint, SecretKey};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn recover_secret() {
    let secret = Shamir::recover_secret(vec![
        "015a1b89f07eba456da29cd428682e2b5f".to_string(),
        "03d46baf922862068db4306d083b108db4".to_string(),
        "05b3fe61724893cf1fe8989637a770dada".to_string(),
    ])
    .unwrap();
    assert_eq!(b"Hello World".to_vec(), secret);
}
