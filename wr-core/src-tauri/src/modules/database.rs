use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::sync::Arc;
use crate::modules::crypto::CryptoManager;
use sha2::{Sha256, Digest};

pub struct DatabaseManager {
    pool: Pool<Sqlite>,
    crypto: Arc<CryptoManager>,
}

impl DatabaseManager {
    pub async fn new(crypto: Arc<CryptoManager>) -> Self {
        let db_url = "sqlite:masterpiece_data.db?mode=rwc";
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
            .expect("Failed to connect to SQLite");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS curriculum (
                id TEXT PRIMARY KEY,
                content_encrypted TEXT NOT NULL,
                integrity_hash TEXT NOT NULL
            )"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS performance_logs (
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                metric_type TEXT,
                value_encrypted TEXT
            )"
        ).execute(&pool).await.unwrap();

        Self { pool, crypto }
    }

    pub async fn store_curriculum(&self, id: &str, content: &[u8]) {
        let encrypted = self.crypto.encrypt(content);

        let mut hasher = Sha256::new();
        hasher.update(content);
        let hash = hex::encode(hasher.finalize());

        sqlx::query("INSERT OR REPLACE INTO curriculum (id, content_encrypted, integrity_hash) VALUES (?, ?, ?)")
            .bind(id)
            .bind(encrypted)
            .bind(hash)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    pub async fn get_curriculum(&self, id: &str) -> Result<String, String> {
        let row: (String, String) = sqlx::query_as("SELECT content_encrypted, integrity_hash FROM curriculum WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let decrypted = self.crypto.decrypt(&row.0)?;

        let mut hasher = Sha256::new();
        hasher.update(&decrypted);
        let hash = hex::encode(hasher.finalize());

        if hash != row.1 {
            return Err("INTEGRITY HASH MISMATCH. PROTOCOL VOID.".to_string());
        }

        Ok(String::from_utf8(decrypted).map_err(|_| "Invalid UTF-8".to_string())?)
    }
}
