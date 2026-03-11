//! Nonce database operations.

use crate::db::DbPool;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Nonce {
    pub nonce: String,
    pub expire: Option<DateTime<Utc>>,
}

pub struct Nonces;

impl Nonces {
    pub async fn insert(pool: &DbPool, nonce: &str, expire: DateTime<Utc>) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO nonces (nonce, expire) VALUES ($1, $2)")
            .bind(nonce)
            .bind(expire)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &DbPool, nonce: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM nonces WHERE nonce = $1")
            .bind(nonce)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete_expired(pool: &DbPool) -> anyhow::Result<u64> {
        let result = sqlx::query("DELETE FROM nonces WHERE expire < now()")
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn select(pool: &DbPool, nonce: &str) -> anyhow::Result<Option<Nonce>> {
        let result = sqlx::query_as::<_, Nonce>("SELECT nonce, expire FROM nonces WHERE nonce = $1")
            .bind(nonce)
            .fetch_optional(pool)
            .await?;
        Ok(result)
    }

    pub async fn exists(pool: &DbPool, nonce: &str) -> anyhow::Result<bool> {
        let result = sqlx::query("SELECT 1 FROM nonces WHERE nonce = $1")
            .bind(nonce)
            .fetch_optional(pool)
            .await?;
        Ok(result.is_some())
    }
}
