use ring::{aead, pbkdf2, rand};
use ring::rand::SecureRandom;
use std::num::NonZeroU32;
use crate::infrastructure::error::{AppResult, encryption_error};
use crate::infrastructure::logging::Logger;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;
const ITERATIONS: u32 = 100_000;

pub struct EncryptionService {
    rng: rand::SystemRandom,
}

impl EncryptionService {
    pub fn new() -> Self {
        Logger::info("EncryptionService", "Initializing encryption service");
        Self {
            rng: rand::SystemRandom::new(),
        }
    }

    fn derive_key(&self, password: &str, salt: &[u8]) -> AppResult<[u8; KEY_LEN]> {
        let mut key = [0u8; KEY_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(ITERATIONS).unwrap(),
            salt,
            password.as_bytes(),
            &mut key,
        );
        Ok(key)
    }

    fn generate_salt(&self) -> AppResult<[u8; SALT_LEN]> {
        let mut salt = [0u8; SALT_LEN];
        self.rng
            .fill(&mut salt)
            .map_err(|e| encryption_error(format!("Failed to generate salt: {:?}", e)))?;
        Ok(salt)
    }

    fn generate_nonce(&self) -> AppResult<[u8; NONCE_LEN]> {
        let mut nonce = [0u8; NONCE_LEN];
        self.rng
            .fill(&mut nonce)
            .map_err(|e| encryption_error(format!("Failed to generate nonce: {:?}", e)))?;
        Ok(nonce)
    }

    pub fn encrypt(&self, data: &[u8], password: &str) -> AppResult<Vec<u8>> {
        Logger::info("EncryptionService", "Starting encryption process");

        // Generate salt and derive key
        let salt = self.generate_salt()?;
        let key = self.derive_key(password, &salt)?;

        // Create sealing key
        let sealing_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key)
            .map_err(|e| encryption_error(format!("Failed to create sealing key: {:?}", e)))?;
        let sealing_key = aead::LessSafeKey::new(sealing_key);

        // Generate nonce
        let nonce = self.generate_nonce()?;
        let nonce = aead::Nonce::assume_unique_for_key(nonce);

        // Encrypt data
        let mut in_out = data.to_vec();
        sealing_key
            .seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|e| encryption_error(format!("Encryption failed: {:?}", e)))?;

        // Combine salt, nonce, and encrypted data
        let mut result = Vec::with_capacity(SALT_LEN + NONCE_LEN + in_out.len());
        result.extend_from_slice(&salt);
        result.extend_from_slice(&nonce.as_ref());
        result.extend_from_slice(&in_out);

        Logger::info("EncryptionService", "Encryption completed successfully");
        Ok(result)
    }

    pub fn decrypt(&self, encrypted_data: &[u8], password: &str) -> AppResult<Vec<u8>> {
        Logger::info("EncryptionService", "Starting decryption process");

        if encrypted_data.len() < SALT_LEN + NONCE_LEN + aead::AES_256_GCM.tag_len() {
            return Err(encryption_error("Invalid encrypted data length"));
        }

        // Extract salt and derive key
        let salt = &encrypted_data[..SALT_LEN];
        let key = self.derive_key(password, salt)?;

        // Create opening key
        let opening_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key)
            .map_err(|e| encryption_error(format!("Failed to create opening key: {:?}", e)))?;
        let opening_key = aead::LessSafeKey::new(opening_key);

        // Extract nonce
        let nonce_start = SALT_LEN;
        let nonce_end = nonce_start + NONCE_LEN;
        let nonce = aead::Nonce::assume_unique_for_key(
            encrypted_data[nonce_start..nonce_end]
                .try_into()
                .map_err(|_| encryption_error("Invalid nonce"))?,
        );

        // Extract encrypted data and decrypt
        let mut in_out = encrypted_data[nonce_end..].to_vec();
        let decrypted_data = opening_key
            .open_in_place(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|e| encryption_error(format!("Decryption failed: {:?}", e)))?;

        Logger::info("EncryptionService", "Decryption completed successfully");
        Ok(decrypted_data.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_encryption_decryption() {
        let service = EncryptionService::new();
        let data = b"Hello, World!";
        let password = "Password123!";

        // Encrypt data
        let encrypted = service.encrypt(data, password).unwrap();
        assert_ne!(encrypted, data);

        // Decrypt data
        let decrypted = service.decrypt(&encrypted, password).unwrap();
        assert_eq!(decrypted, data);
    }

    #[wasm_bindgen_test]
    fn test_wrong_password() {
        let service = EncryptionService::new();
        let data = b"Hello, World!";
        let password = "Password123!";
        let wrong_password = "WrongPassword123!";

        // Encrypt with correct password
        let encrypted = service.encrypt(data, password).unwrap();

        // Attempt to decrypt with wrong password
        let result = service.decrypt(&encrypted, wrong_password);
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_invalid_encrypted_data() {
        let service = EncryptionService::new();
        let invalid_data = vec![0; 10]; // Too short to be valid
        let password = "Password123!";

        let result = service.decrypt(&invalid_data, password);
        assert!(result.is_err());
    }
}
