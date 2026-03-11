//! Video database operations.

use crate::db::DbPool;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Video {
    pub id: String,
    pub info: Option<String>,
    pub updated: Option<DateTime<Utc>>,
}

pub struct Videos;

impl Videos {
    pub async fn insert(pool: &DbPool, video: &Video) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO videos (id, info, updated) VALUES ($1, $2, $3) ON CONFLICT (id) DO NOTHING",
        )
        .bind(&video.id)
        .bind(&video.info)
        .bind(&video.updated)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &DbPool, id: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM videos WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete_expired(pool: &DbPool) -> anyhow::Result<u64> {
        let result = sqlx::query("DELETE FROM videos WHERE updated < (now() - interval '6 hours')")
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn update(pool: &DbPool, video: &Video) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE videos SET (id, info, updated) = ($1, $2, $3) WHERE id = $1",
        )
        .bind(&video.id)
        .bind(&video.info)
        .bind(&video.updated)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn select(pool: &DbPool, id: &str) -> anyhow::Result<Option<Video>> {
        let video = sqlx::query_as::<_, Video>(
            "SELECT id, info, updated FROM videos WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(video)
    }
}
