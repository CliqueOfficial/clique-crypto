mod utils;

use aes_gcm::{Aes256Gcm, Key, KeyInit};
use k256::{
    ecdh::diffie_hellman, elliptic_curve::sec1::FromEncodedPoint, EncodedPoint, PublicKey,
    SecretKey,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AES {
    inner: Aes256Gcm,
}

#[wasm_bindgen]
impl AES {
    pub fn with_ecdh(sk: &[u8], pk: &[u8]) -> Self {
        utils::set_panic_hook();

        assert_eq!(sk.len(), 32);
        assert_eq!(pk.len(), 65);

        let sk = SecretKey::from_slice(sk).unwrap();
        let point = EncodedPoint::from_bytes(pk).unwrap();
        let pk = PublicKey::from_encoded_point(&point).unwrap();

        let shared_secrets = diffie_hellman(&sk.to_nonzero_scalar(), pk.as_affine());
        let mut key = Key::<Aes256Gcm>::default();
        shared_secrets
            .extract::<sha2::Sha256>(None)
            .expand(&[], &mut key)
            .unwrap();

        Self {
            inner: Aes256Gcm::new(&key),
        }
    }
}
