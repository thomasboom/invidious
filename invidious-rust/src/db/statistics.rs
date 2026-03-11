//! Statistics database operations.

use crate::db::DbPool;
use sqlx::{FromRow, Row};

#[derive(Debug, Clone, FromRow)]
pub struct Statistics {
    pub id: i64,
    pub users: Option<i64>,
    pub videos: Option<i64>,
    pub subscriptions: Option<i64>,
    pub updates_per_hour: Option<i64>,
}

pub struct StatisticsDB;

impl StatisticsDB {
    pub async fn get(pool: &DbPool) -> anyhow::Result<Option<Statistics>> {
        let stats = sqlx::query_as::<_, Statistics>(
            "SELECT id, users, videos, subscriptions, updates_per_hour FROM statistics LIMIT 1"
        )
        .fetch_optional(pool)
        .await?;
        Ok(stats)
    }

    pub async fn update(pool: &DbPool, users: i64, videos: i64, subscriptions: i64) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO statistics (id, users, videos, subscriptions, updates_per_hour) VALUES (1, $1, $2, $3, 0) ON CONFLICT (id) DO UPDATE SET users = $1, videos = $2, subscriptions = $3")
            .bind(users)
            .bind(videos)
            .bind(subscriptions)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn count_users(pool: &DbPool) -> anyhow::Result<i64> {
        let row = sqlx::query("SELECT count(*) FROM users")
            .fetch_one(pool)
            .await?;
        Ok(row.get::<i64, _>(0))
    }

    pub async fn count_videos(pool: &DbPool) -> anyhow::Result<i64> {
        let row = sqlx::query("SELECT count(*) FROM videos")
            .fetch_one(pool)
            .await?;
        Ok(row.get::<i64, _>(0))
    }

    pub async fn count_subscriptions(pool: &DbPool) -> anyhow::Result<i64> {
        let row = sqlx::query("SELECT count(*) FROM (SELECT unnest(subscriptions) FROM users) AS sub")
            .fetch_one(pool)
            .await?;
        Ok(row.get::<i64, _>(0))
    }

    pub async fn count_channels(pool: &DbPool) -> anyhow::Result<i64> {
        let row = sqlx::query("SELECT count(*) FROM channels")
            .fetch_one(pool)
            .await?;
        Ok(row.get::<i64, _>(0))
    }

    pub async fn count_playlists(pool: &DbPool) -> anyhow::Result<i64> {
        let row = sqlx::query("SELECT count(*) FROM playlists")
            .fetch_one(pool)
            .await?;
        Ok(row.get::<i64, _>(0))
    }
}
