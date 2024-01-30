use aes_gcm::aead::{Aead, Nonce};
use aes_gcm::{Aes256Gcm, Key, KeyInit};
use k256::{
    ecdh::diffie_hellman, elliptic_curve::sec1::FromEncodedPoint, EncodedPoint, PublicKey,
    SecretKey,
};
use sha2::{Digest, Sha256};

use wasm_bindgen::prelude::*;

use crate::utils;

#[wasm_bindgen]
#[allow(dead_code)]
pub struct AES {
    inner: Aes256Gcm,
}

#[wasm_bindgen]
impl AES {
    #[wasm_bindgen(js_name = withECDH)]
    pub fn with_ecdh(priv_key: &[u8], pub_key: &[u8]) -> Self {
        utils::set_panic_hook();

        assert_eq!(priv_key.len(), 32);
        assert_eq!(pub_key.len(), 65);

        let priv_key = SecretKey::from_slice(priv_key).unwrap();
        let point = EncodedPoint::from_bytes(pub_key).unwrap();
        let pub_key = PublicKey::from_encoded_point(&point).unwrap();

        let shared_secrets = diffie_hellman(&priv_key.to_nonzero_scalar(), pub_key.as_affine());
        let mut key = Key::<Aes256Gcm>::default();
        shared_secrets
            .extract::<sha2::Sha256>(None)
            .expand(&[], &mut key)
            .unwrap();

        Self {
            inner: Aes256Gcm::new(&key),
        }
    }

    #[wasm_bindgen(js_name = withPassword)]
    pub fn with_password(password: &str) -> Self {
        utils::set_panic_hook();
        let key = Sha256::new().chain_update(password).finalize();
        Self {
            inner: Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key)),
        }
    }

    #[wasm_bindgen]
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, JsValue> {
        let nonce = match utils::get_random_buf() {
            Ok(v) => v.to_vec(),
            Err(err) => {
                return Err(JsValue::from(err.to_string()));
            }
        };
        assert_eq!(nonce.len(), 12);
        let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce);
        match self.inner.encrypt(nonce, data) {
            Ok(v) => Ok([nonce, v.as_slice()].concat()),
            Err(err) => Err(JsValue::from(err.to_string())),
        }
    }

    #[wasm_bindgen]
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, JsValue> {
        let nonce = Nonce::<Aes256Gcm>::from_slice(&data[..12]);
        match self.inner.decrypt(nonce, &data[12..]) {
            Ok(v) => Ok(v),
            Err(err) => Err(JsValue::from(err.to_string())),
        }
    }
}
