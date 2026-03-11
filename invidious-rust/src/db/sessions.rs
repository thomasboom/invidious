//! Session database operations.

use crate::db::DbPool;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct SessionId {
    pub id: String,
    pub email: Option<String>,
    pub issued: Option<DateTime<Utc>>,
}

pub struct SessionIds;

impl SessionIds {
    pub async fn insert(pool: &DbPool, sid: &str, email: &str, handle_conflicts: bool) -> anyhow::Result<()> {
        if handle_conflicts {
            sqlx::query("INSERT INTO session_ids (id, email, issued) VALUES ($1, $2, now()) ON CONFLICT (id) DO NOTHING")
                .bind(sid)
                .bind(email)
                .execute(pool)
                .await?;
        } else {
            sqlx::query("INSERT INTO session_ids (id, email, issued) VALUES ($1, $2, now())")
                .bind(sid)
                .bind(email)
                .execute(pool)
                .await?;
        }
        Ok(())
    }

    pub async fn delete_by_sid(pool: &DbPool, sid: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM session_ids WHERE id = $1")
            .bind(sid)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete_by_email(pool: &DbPool, email: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM session_ids WHERE email = $1")
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete_by_sid_and_email(pool: &DbPool, sid: &str, email: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM session_ids WHERE id = $1 AND email = $2")
            .bind(sid)
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn select_email(pool: &DbPool, sid: &str) -> anyhow::Result<Option<String>> {
        let result = sqlx::query_as::<_, SessionId>("SELECT id, email, issued FROM session_ids WHERE id = $1")
            .bind(sid)
            .fetch_optional(pool)
            .await?;
        Ok(result.and_then(|r| r.email))
    }

    pub async fn select_all(pool: &DbPool, email: &str) -> anyhow::Result<Vec<(String, DateTime<Utc>)>> {
        let sessions = sqlx::query_as::<_, SessionId>("SELECT id, email, issued FROM session_ids WHERE email = $1 ORDER BY issued DESC")
            .bind(email)
            .fetch_all(pool)
            .await?;

        Ok(sessions.into_iter().filter_map(|s| s.issued.map(|i| (s.id, i))).collect())
    }
}
