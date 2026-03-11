//! Channel database operations.

use crate::db::DbPool;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Channel {
    pub id: String,
    pub author: Option<String>,
    pub updated: Option<DateTime<Utc>>,
    pub deleted: Option<bool>,
    pub subscribed: Option<DateTime<Utc>>,
}

pub struct Channels;

impl Channels {
    pub async fn insert(pool: &DbPool, channel: &Channel, update_on_conflict: bool) -> anyhow::Result<()> {
        if update_on_conflict {
            sqlx::query(
                "INSERT INTO channels (id, author, updated, deleted, subscribed) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET author = $2, updated = $3",
            )
            .bind(&channel.id)
            .bind(&channel.author)
            .bind(&channel.updated)
            .bind(channel.deleted.unwrap_or(false))
            .bind(&channel.subscribed)
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                "INSERT INTO channels (id, author, updated, deleted, subscribed) VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(&channel.id)
            .bind(&channel.author)
            .bind(&channel.updated)
            .bind(channel.deleted.unwrap_or(false))
            .bind(&channel.subscribed)
            .execute(pool)
            .await?;
        }
        Ok(())
    }

    pub async fn update_author(pool: &DbPool, id: &str, author: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE channels SET updated = now(), author = $1, deleted = false WHERE id = $2")
            .bind(author)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_subscription_time(pool: &DbPool, id: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE channels SET subscribed = now() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_mark_deleted(pool: &DbPool, id: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE channels SET updated = now(), deleted = true WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn select(pool: &DbPool, id: &str) -> anyhow::Result<Option<Channel>> {
        let channel = sqlx::query_as::<_, Channel>(
            "SELECT id, author, updated, deleted, subscribed FROM channels WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(channel)
    }

    pub async fn select_many(pool: &DbPool, ids: &[String]) -> anyhow::Result<Vec<Channel>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let channels = sqlx::query_as::<_, Channel>(
            "SELECT id, author, updated, deleted, subscribed FROM channels WHERE id = ANY($1)"
        )
        .bind(ids)
        .fetch_all(pool)
        .await?;
        Ok(channels)
    }
}
