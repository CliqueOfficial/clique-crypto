//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use std::vec;

use wasm_bindgen_test::*;

use clique_crypto::shamir::Shamir;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn shamir() {
    let secret = b"Hello World".to_vec();
    let shares = Shamir::generate_shares(&secret, 3, 2).unwrap();
    assert_eq!(
        secret,
        Shamir::recover_secret(vec![shares[0].clone(), shares[1].clone()]).unwrap()
    );
    assert_eq!(
        secret,
        Shamir::recover_secret(vec![shares[0].clone(), shares[2].clone()]).unwrap()
    );
    assert_eq!(
        secret,
        Shamir::recover_secret(vec![shares[1].clone(), shares[2].clone()]).unwrap()
    );
}
