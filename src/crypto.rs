use aes_gcm::Aes256Gcm;
use aes_gcm::KeyInit;
use aes_gcm::Nonce;
use aes_gcm::aead::Aead;
use rand::Rng;
use rand::RngCore;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::time::SystemTime;

const TOKEN_CACHE_DIR: &str = ".spinup";
const TOKEN_CACHE_FILE: &str = "token_cache.enc";
pub const TOKEN_CACHE_DURATION_SECS: u64 = 24 * 60 * 60;

#[derive(Debug)]
pub struct EncryptedToken {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub encrypted_at: u64,
}

#[derive(Debug)]
pub struct DecryptedToken {
    pub token: String,
    pub encrypted_at: u64,
    pub is_expired: bool,
}

pub struct EncryptionAPI {
    cipher: Option<Aes256Gcm>,
}

impl EncryptionAPI {
    pub fn new() -> Self {
        Self { cipher: None }
    }

    fn get_cache_dir() -> std::path::PathBuf {
        let home = std::env::var("HOME")
            .unwrap_or_else(|_| std::env::current_dir().unwrap().display().to_string());
        std::path::PathBuf::from(format!("{}/{}", home, TOKEN_CACHE_DIR))
    }

    fn get_or_create_key() -> Result<[u8; 32], Box<dyn std::error::Error>> {
        let cache_dir = Self::get_cache_dir();
        let key_file = cache_dir.join("master_key.enc");

        if key_file.exists() {
            let mut key = [0u8; 32];
            let mut file = fs::File::open(&key_file)?;
            file.read_exact(&mut key)?;
            return Ok(key);
        }

        let mut key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);

        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }

        let mut file = fs::File::create(&key_file)?;
        file.write_all(&key)?;
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o600);
        file.set_permissions(perms)?;

        Ok(key)
    }

    fn init_cipher(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.cipher.is_none() {
            let key = Self::get_or_create_key()?;
            match Aes256Gcm::new_from_slice(&key) {
                Ok(c) => {
                    self.cipher = Some(c);
                }
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to create cipher: {}", e),
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn encrypt_token(
        &mut self,
        token: &str,
    ) -> Result<EncryptedToken, Box<dyn std::error::Error>> {
        self.init_cipher()?;

        let nonce: [u8; 12] = rand::thread_rng().r#gen();
        let cipher = self.cipher.as_ref().unwrap();

        let ciphertext = cipher
            .encrypt(&Nonce::from(nonce), token.as_bytes())
            .map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Encryption failed: {}", e),
                ))
            })?;

        let encrypted_at = SystemTime::UNIX_EPOCH.elapsed()?.as_secs();

        Ok(EncryptedToken {
            ciphertext,
            nonce,
            encrypted_at,
        })
    }

    pub fn decrypt_token(
        &mut self,
        encrypted: &EncryptedToken,
    ) -> Result<DecryptedToken, Box<dyn std::error::Error>> {
        self.init_cipher()?;

        let cipher = self.cipher.as_ref().unwrap();

        let plaintext = cipher
            .decrypt(
                &Nonce::from(encrypted.nonce),
                encrypted.ciphertext.as_slice(),
            )
            .map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Decryption failed: {}", e),
                ))
            })?;

        let token = String::from_utf8(plaintext).map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Invalid token encoding: {}", e),
            ))
        })?;

        let now = SystemTime::UNIX_EPOCH.elapsed()?.as_secs();
        let is_expired = now - encrypted.encrypted_at > TOKEN_CACHE_DURATION_SECS;

        Ok(DecryptedToken {
            token,
            encrypted_at: encrypted.encrypted_at,
            is_expired,
        })
    }

    pub fn store_cached_token(&mut self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted = self.encrypt_token(token)?;

        let cache_dir = Self::get_cache_dir();
        let cache_file = cache_dir.join(TOKEN_CACHE_FILE);

        let mut file = fs::File::create(&cache_file)?;
        file.write_all(&encrypted.encrypted_at.to_le_bytes())?;
        file.write_all(&encrypted.nonce)?;
        file.write_all(&(encrypted.ciphertext.len() as u32).to_le_bytes())?;
        file.write_all(&encrypted.ciphertext)?;

        Ok(())
    }

    pub fn get_cached_token(
        &mut self,
    ) -> Result<Option<DecryptedToken>, Box<dyn std::error::Error>> {
        let cache_dir = Self::get_cache_dir();
        let cache_file = cache_dir.join(TOKEN_CACHE_FILE);

        if !cache_file.exists() {
            return Ok(None);
        }

        let mut file = fs::File::open(&cache_file)?;

        let mut encrypted_at_bytes = [0u8; 8];
        file.read_exact(&mut encrypted_at_bytes)?;
        let encrypted_at = u64::from_le_bytes(encrypted_at_bytes);

        let mut nonce = [0u8; 12];
        file.read_exact(&mut nonce)?;

        let mut len_bytes = [0u8; 4];
        file.read_exact(&mut len_bytes)?;
        let len = u32::from_le_bytes(len_bytes) as usize;

        let mut ciphertext = vec![0u8; len];
        file.read_exact(&mut ciphertext)?;

        self.decrypt_token(&EncryptedToken {
            ciphertext,
            nonce,
            encrypted_at,
        })
        .map(Some)
    }

    pub fn clear_cached_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cache_dir = Self::get_cache_dir();
        let cache_file = cache_dir.join(TOKEN_CACHE_FILE);

        if cache_file.exists() {
            fs::remove_file(&cache_file)?;
        }

        self.cipher = None;

        Ok(())
    }

    pub fn encrypt_secret(&mut self, secret: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.init_cipher()?;

        let nonce: [u8; 12] = rand::thread_rng().r#gen();
        let cipher = self.cipher.as_ref().unwrap();

        let ciphertext = cipher
            .encrypt(&Nonce::from(nonce), secret.as_bytes())
            .map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Encryption failed: {}", e),
                ))
            })?;

        let mut result = nonce.to_vec();
        result.extend_from_slice(&(ciphertext.len() as u32).to_le_bytes());
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    pub fn decrypt_secret(&mut self, data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        if data.len() < 16 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid encrypted data",
            )));
        }

        let nonce: [u8; 12] = data[..12].try_into().unwrap();
        let len = u32::from_le_bytes(data[12..16].try_into().unwrap()) as usize;
        let ciphertext = &data[16..16 + len];

        let cipher = self.cipher.as_ref().unwrap();

        let plaintext = cipher
            .decrypt(&Nonce::from(nonce), ciphertext)
            .map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Decryption failed: {}", e),
                ))
            })?;

        Ok(String::from_utf8(plaintext).map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Invalid secret encoding: {}", e),
            ))
        })?)
    }

    pub fn generate_signature(
        &mut self,
        data: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.init_cipher()?;

        let mut signature = [0u8; 64];
        rand::thread_rng().fill_bytes(&mut signature);

        let mut input = data.as_bytes().to_vec();
        input.extend_from_slice(&signature[..32]);

        self.encrypt_secret(&String::from_utf8(input)?)
    }

    pub fn verify_signature(
        &mut self,
        data: &str,
        signature: &[u8],
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let decrypted = self.decrypt_secret(signature)?;

        if decrypted.len() < 32 {
            return Ok(false);
        }

        let received_data = &decrypted[32..];
        let expected_data = data.as_bytes();

        if received_data.len() != expected_data.len() {
            return Ok(false);
        }

        let received_bytes = received_data.as_bytes();
        Ok(received_bytes == expected_data)
    }
}

impl Default for EncryptionAPI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_token() {
        let mut api = EncryptionAPI::new();
        let token = "ghp_test_token_12345";

        let encrypted = api.encrypt_token(token).unwrap();
        let decrypted = api.decrypt_token(&encrypted).unwrap();

        assert_eq!(decrypted.token, token);
        assert!(!decrypted.is_expired);
    }

    #[test]
    fn test_store_get_cached_token() {
        let mut api = EncryptionAPI::new();
        let token = "ghp_cached_token_67890";

        api.store_cached_token(token).unwrap();
        let cached = api.get_cached_token().unwrap().unwrap();

        assert_eq!(cached.token, token);
        assert!(!cached.is_expired);
    }

    #[test]
    fn test_encrypt_decrypt_secret() {
        let mut api = EncryptionAPI::new();
        let secret = "super-secret-url-https://github.com/user/repo";

        let encrypted = api.encrypt_secret(secret).unwrap();
        let decrypted = api.decrypt_secret(&encrypted).unwrap();

        assert_eq!(decrypted, secret);
    }
}
