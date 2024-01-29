//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use clique_crypto::AES;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    use k256::{
        elliptic_curve::{bigint::Uint, sec1::ToEncodedPoint},
        NonZeroScalar, PublicKey, SecretKey,
    };

    let alice = [1u8; 32];
    let sk = SecretKey::from_slice(&alice).unwrap();
    let pk = sk.public_key();

    AES::with_ecdh(
        sk.to_bytes().as_slice(),
        pk.to_encoded_point(false).as_bytes(),
    );
}
