#[cfg(test)]
mod tests {
    use crate::modules::crypto::CryptoManager;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_encryption_decryption() {
        let dir = tempdir().unwrap();
        let crypto = CryptoManager::new(dir.path().to_path_buf()).unwrap();

        let original_data = b"Masterpiece Protocol 2024";
        let encrypted = crypto.encrypt(original_data).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();

        assert_eq!(original_data, decrypted.as_slice());
    }

    #[test]
    fn test_unique_nonces() {
        let dir = tempdir().unwrap();
        let crypto = CryptoManager::new(dir.path().to_path_buf()).unwrap();

        let data = b"Test Data";
        let enc1 = crypto.encrypt(data).unwrap();
        let enc2 = crypto.encrypt(data).unwrap();

        assert_ne!(enc1, enc2);
    }

    #[test]
    fn test_salt_persistence() {
        let dir = tempdir().unwrap();
        let app_data = dir.path().to_path_buf();

        {
            let _ = CryptoManager::new(app_data.clone()).unwrap();
        }

        let salt_file = app_data.join("installation.salt");
        assert!(salt_file.exists());

        let salt_content = fs::read(&salt_file).unwrap();
        assert_eq!(salt_content.len(), 32);
    }
}
