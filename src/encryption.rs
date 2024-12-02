use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use crate::ProtocolError;
use bytes::Bytes;
use rand::Rng;

/// Handles encryption and decryption of messages using AES-256-GCM
pub struct Encryptor {
    cipher: Aes256Gcm,
}

impl Encryptor {
    /// Creates a new encryptor with the given 32-byte key
    pub fn new(key: &[u8; 32]) -> Self {
        Self {
            cipher: Aes256Gcm::new(key.into()),
        }
    }

    /// Encrypts data with a random nonce and returns the concatenated nonce + ciphertext
    pub fn encrypt(&self, data: &[u8]) -> Result<Bytes, ProtocolError> {
        let mut rng = rand::thread_rng();
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, data)
            .map_err(|e| ProtocolError::EncryptionError(e.to_string()))?;

        // Combine nonce and ciphertext
        let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        
        Ok(Bytes::from(result))
    }

    /// Decrypts data that was encrypted with encrypt()
    pub fn decrypt(&self, data: &[u8]) -> Result<Bytes, ProtocolError> {
        if data.len() < 12 {
            return Err(ProtocolError::EncryptionError("Data too short".into()));
        }

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| ProtocolError::EncryptionError(e.to_string()))?;

        Ok(Bytes::from(plaintext))
    }

    /// Helper function to generate a random encryption key
    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        rand::thread_rng().fill(&mut key);
        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_roundtrip() {
        let key = Encryptor::generate_key();
        let encryptor = Encryptor::new(&key);
        let data = b"Hello, World!";
        
        let encrypted = encryptor.encrypt(data).unwrap();
        let decrypted = encryptor.decrypt(&encrypted).unwrap();
        
        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_encryption_tamper_detection() {
        let key = Encryptor::generate_key();
        let encryptor = Encryptor::new(&key);
        let encrypted = encryptor.encrypt(b"valid").unwrap();
        
        // Create a modified version of the encrypted data
        let mut tampered = encrypted.to_vec();
        tampered[12] ^= 1; // Corrupt one byte
        
        // Decryption should fail
        assert!(encryptor.decrypt(&tampered).is_err());
    }

    #[test]
    fn test_encryption_different_nonces() {
        let key = Encryptor::generate_key();
        let encryptor = Encryptor::new(&key);
        
        let data = b"Same data";
        let encrypted1 = encryptor.encrypt(data).unwrap();
        let encrypted2 = encryptor.encrypt(data).unwrap();
        
        // Same data should produce different ciphertexts due to random nonces
        assert_ne!(encrypted1, encrypted2);
    }

    #[test]
    fn test_decryption_invalid_data() {
        let key = Encryptor::generate_key();
        let encryptor = Encryptor::new(&key);
        
        // Test data too short
        assert!(encryptor.decrypt(&[1, 2, 3]).is_err());
    }

    #[test]
    fn test_key_generation() {
        let key1 = Encryptor::generate_key();
        let key2 = Encryptor::generate_key();
        
        assert_ne!(key1, key2); // Keys should be random
        assert_eq!(key1.len(), 32); // Key should be correct length
    }
}