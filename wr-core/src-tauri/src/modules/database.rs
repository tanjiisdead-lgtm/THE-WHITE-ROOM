use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::sync::Arc;
use crate::modules::crypto::CryptoManager;
use sha2::{Sha256, Digest};
use std::path::PathBuf;

pub struct DatabaseManager {
    pool: Pool<Sqlite>,
    crypto: Arc<CryptoManager>,
}

impl DatabaseManager {
    pub async fn new(crypto: Arc<CryptoManager>, app_data_path: PathBuf) -> Result<Self, String> {
        let db_path = app_data_path.join("masterpiece.db");
        let db_url = format!("sqlite:{}?mode=rwc", db_path.to_string_lossy());

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .map_err(|e| format!("Failed to connect to SQLite: {}", e))?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS curriculum (
                id TEXT PRIMARY KEY,
                content_encrypted TEXT NOT NULL,
                integrity_hash TEXT NOT NULL
            )"
        ).execute(&pool).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS performance_logs (
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                metric_type TEXT,
                value_encrypted TEXT
            )"
        ).execute(&pool).await.map_err(|e| e.to_string())?;

        Ok(Self { pool, crypto } )
    }

    pub async fn store_curriculum(&self, id: &str, content: &[u8]) -> Result<(), String> {
        let encrypted = self.crypto.encrypt(content)?;

        let mut hasher = Sha256::new();
        hasher.update(content);
        let hash = hex::encode(hasher.finalize());

        sqlx::query("INSERT OR REPLACE INTO curriculum (id, content_encrypted, integrity_hash) VALUES (?, ?, ?)")
            .bind(id)
            .bind(encrypted)
            .bind(hash)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_curriculum(&self, id: &str) -> Result<String, String> {
        let row: (String, String) = sqlx::query_as("SELECT content_encrypted, integrity_hash FROM curriculum WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Curriculum not found: {}", e))?;

        // In a strictly lenient mode, we still verify integrity but handle errors gracefully
        let decrypted = self.crypto.decrypt(&row.0)?;

        let mut hasher = Sha256::new();
        hasher.update(&decrypted);
        let hash = hex::encode(hasher.finalize());

        if hash != row.1 {
            return Err("Data integrity verification failed.".to_string());
        }

        String::from_utf8(decrypted).map_err(|_| "Invalid UTF-8 content".to_string())
    }

    pub async fn clear_logs(&self) -> Result<(), String> {
        sqlx::query("DELETE FROM performance_logs").execute(&self.pool).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
