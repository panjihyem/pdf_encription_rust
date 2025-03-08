use ring::{aead, pbkdf2};
use crate::domain::entities::{PdfDocument, EncryptionParams};
use rand::{thread_rng, RngCore};

const KEY_SIZE: usize = 256 / 8; // 256 bits in bytes
const PBKDF2_ITERATIONS: u32 = 100_000;
const SALT_SIZE: usize = 16;
const NONCE_SIZE: usize = 12;

pub fn encrypt_pdf(pdf: &PdfDocument, password: &str) -> Result<PdfDocument, String> {
    // Generate salt for PBKDF2
    let mut salt = [0u8; SALT_SIZE];
    thread_rng().fill_bytes(&mut salt);

    // Derive key using PBKDF2
    let mut key = [0u8; KEY_SIZE];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
        &salt,
        password.as_bytes(),
        &mut key,
    );

    // Create unbound key for AES-GCM
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key)
        .map_err(|_| "Failed to create encryption key".to_string())?;

    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

    // Create AES-GCM sealing key
    let sealing_key = aead::LessSafeKey::new(unbound_key);

    // Encrypt content
    let mut in_out = pdf.get_content().to_vec();
    sealing_key
        .seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
        .map_err(|_| "Encryption failed".to_string())?;

    // Create new encrypted PDF document
    let mut encrypted_pdf = pdf.clone();
    encrypted_pdf.set_content(in_out);
    
    // Set encryption parameters
    let params = EncryptionParams {
        key_size: KEY_SIZE * 8,
        iterations: PBKDF2_ITERATIONS as usize,
        salt_size: SALT_SIZE,
        nonce_size: NONCE_SIZE,
    };
    
    encrypted_pdf.set_encryption_params(params);

    Ok(encrypted_pdf)
}

pub fn decrypt_pdf(pdf: &PdfDocument, password: &str) -> Result<PdfDocument, String> {
    if !pdf.is_encrypted() {
        return Err("Document is not encrypted".to_string());
    }

    let params = pdf.get_encryption_params();
    
    // Derive key using PBKDF2
    let mut key = [0u8; KEY_SIZE];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
        &[0u8; SALT_SIZE], // Use stored salt in production
        password.as_bytes(),
        &mut key,
    );

    // Create unbound key for AES-GCM
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key)
        .map_err(|_| "Failed to create decryption key".to_string())?;

    // Create AES-GCM opening key
    let opening_key = aead::LessSafeKey::new(unbound_key);

    // Extract nonce from encrypted content
    let nonce_bytes = [0u8; NONCE_SIZE]; // Use stored nonce in production
    let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

    // Decrypt content
    let mut in_out = pdf.get_content().to_vec();
    opening_key
        .open_in_place(nonce, aead::Aad::empty(), &mut in_out)
        .map_err(|_| "Decryption failed - invalid password".to_string())?;

    // Remove authentication tag
    in_out.truncate(in_out.len() - aead::AES_256_GCM.tag_len());

    // Create new decrypted PDF document
    let mut decrypted_pdf = pdf.clone();
    decrypted_pdf.set_content(in_out);
    Ok(decrypted_pdf)
}
