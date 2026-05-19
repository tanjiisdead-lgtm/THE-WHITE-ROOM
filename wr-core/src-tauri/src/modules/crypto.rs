use ring::{aead, pbkdf2, rand::{self, SecureRandom}};
use std::num::NonZeroU32;
use base64::{Engine as _, engine::general_purpose};

#[cfg(target_os = "windows")]
use windows::{
    core::*,
    Win32::System::Registry::*,
    Win32::Foundation::*,
};

pub struct CryptoManager {
    key: aead::LessSafeKey,
    rng: rand::SystemRandom,
}

impl CryptoManager {
    pub fn new() -> Self {
        let hardware_id = Self::get_hardware_id();
        let salt = b"WR_CORE_SALT_2024_PROD_V2";
        let mut stretched_key = [0u8; 32];

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(200_000).unwrap(),
            salt,
            hardware_id.as_bytes(),
            &mut stretched_key,
        );

        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &stretched_key).expect("Invalid key");
        let key = aead::LessSafeKey::new(unbound_key);
        let rng = rand::SystemRandom::new();

        Self { key, rng }
    }

    fn get_hardware_id() -> String {
        #[cfg(target_os = "windows")]
        {
            let mut id = String::new();
            // 1. Query MachineGuid from Registry
            unsafe {
                let subkey = w!("SOFTWARE\\Microsoft\\Cryptography");
                let mut hkey = HKEY::default();
                if RegOpenKeyExW(HKEY_LOCAL_MACHINE, subkey, 0, KEY_READ, &mut hkey).is_ok() {
                    let mut data = [0u16; 256];
                    let mut size = (data.len() * 2) as u32;
                    if RegQueryValueExW(hkey, w!("MachineGuid"), None, None, Some(data.as_mut_ptr() as *mut u8), Some(&mut size)).is_ok() {
                        id.push_str(&String::from_utf16_lossy(&data[..((size/2)-1) as usize]));
                    }
                    let _ = RegCloseKey(hkey);
                }
            }
            if id.is_empty() { "FALLBACK_WR_ID".to_string() } else { id }
        }
        #[cfg(not(target_os = "windows"))]
        {
            "DEV_ENVIRONMENT_ID".to_string()
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> String {
        let mut nonce_bytes = [0u8; 12];
        self.rng.fill(&mut nonce_bytes).expect("RNG failure");
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

        let mut in_out = data.to_vec();
        self.key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out).expect("Encryption failed");

        let mut final_payload = nonce_bytes.to_vec();
        final_payload.extend_from_slice(&in_out);
        general_purpose::STANDARD.encode(final_payload)
    }

    pub fn decrypt(&self, encrypted_b64: &str) -> Result<Vec<u8>, String> {
        let encrypted_data = general_purpose::STANDARD.decode(encrypted_b64)
            .map_err(|e| e.to_string())?;

        if encrypted_data.len() < 28 {
            return Err("Invalid payload length".to_string());
        }

        let (nonce_bytes, ciphertext_with_tag) = encrypted_data.split_at(12);
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes.try_into().unwrap());

        let mut in_out = ciphertext_with_tag.to_vec();
        let decrypted_data = self.key.open_in_place(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|_| "Decryption failed".to_string())?;

        Ok(decrypted_data.to_vec())
    }
}
