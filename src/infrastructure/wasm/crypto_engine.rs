use crate::application::use_cases::CryptoPort;
use ring::{aead, pbkdf2, rand};
use std::error::Error;
use std::num::NonZeroU32;
use wasm_bindgen::prelude::*;

pub struct WasmCryptoEngine;

impl WasmCryptoEngine {
    pub fn new() -> Self {
        Self
    }

    fn derive_key(&self, password: &str, salt: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut key = vec![0; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(100_000).unwrap(),
            salt,
            password.as_bytes(),
            &mut key,
        );
        Ok(key)
    }
}

#[async_trait(?Send)]
impl CryptoPort for WasmCryptoEngine {
    async fn encrypt(&self, data: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let rng = rand::SystemRandom::new();
        let salt: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect();
        let key = self.derive_key(password, &salt)?;
        
        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key)
            .map_err(|_| "Failed to create key")?;
        let nonce = aead::Nonce::assume_unique_for_key([0u8; 12]);
        let aad = aead::Aad::empty();
        
        let mut in_out = data.to_vec();
        let sealing_key = aead::LessSafeKey::new(unbound_key);
        sealing_key
            .seal_in_place_append_tag(nonce, aad, &mut in_out)
            .map_err(|_| "Encryption failed")?;

        let mut result = salt.to_vec();
        result.extend(in_out);
        Ok(result)
    }

    async fn decrypt(&self, data: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        if data.len() < 16 {
            return Err("Invalid encrypted data".into());
        }

        let (salt, encrypted) = data.split_at(16);
        let key = self.derive_key(password, salt)?;

        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key)
            .map_err(|_| "Failed to create key")?;
        let nonce = aead::Nonce::assume_unique_for_key([0u8; 12]);
        let aad = aead::Aad::empty();

        let mut in_out = encrypted.to_vec();
        let opening_key = aead::LessSafeKey::new(unbound_key);
        let decrypted = opening_key
            .open_in_place(nonce, aad, &mut in_out)
            .map_err(|_| "Decryption failed")?;

        Ok(decrypted.to_vec())
    }

    async fn validate_password(&self, data: &[u8], password: &str) -> Result<bool, Box<dyn Error>> {
        match self.decrypt(data, password).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
