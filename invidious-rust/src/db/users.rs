//! User database operations.

use crate::db::DbPool;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub updated: Option<DateTime<Utc>>,
    pub notifications: Option<Vec<String>>,
    pub subscriptions: Option<Vec<String>>,
    pub email: String,
    pub preferences: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub watched: Option<Vec<String>>,
    pub feed_needs_update: Option<bool>,
}

pub struct Users;

impl Users {
    pub async fn insert(pool: &DbPool, user: &User, update_on_conflict: bool) -> anyhow::Result<()> {
        if update_on_conflict {
            sqlx::query(
                "INSERT INTO users (updated, notifications, subscriptions, email, preferences, password, token, watched, feed_needs_update) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) ON CONFLICT (email) DO UPDATE SET updated = $1, subscriptions = $3",
            )
            .bind(&user.updated)
            .bind(&user.notifications)
            .bind(&user.subscriptions)
            .bind(&user.email)
            .bind(&user.preferences)
            .bind(&user.password)
            .bind(&user.token)
            .bind(&user.watched)
            .bind(user.feed_needs_update.unwrap_or(false))
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                "INSERT INTO users (updated, notifications, subscriptions, email, preferences, password, token, watched, feed_needs_update) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            )
            .bind(&user.updated)
            .bind(&user.notifications)
            .bind(&user.subscriptions)
            .bind(&user.email)
            .bind(&user.preferences)
            .bind(&user.password)
            .bind(&user.token)
            .bind(&user.watched)
            .bind(user.feed_needs_update.unwrap_or(false))
            .execute(pool)
            .await?;
        }
        Ok(())
    }

    pub async fn delete(pool: &DbPool, email: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM users WHERE email = $1")
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn select_by_email(pool: &DbPool, email: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT updated, notifications, subscriptions, email, preferences, password, token, watched, feed_needs_update FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;
        Ok(user)
    }

    pub async fn select_by_token(pool: &DbPool, token: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT updated, notifications, subscriptions, email, preferences, password, token, watched, feed_needs_update FROM users WHERE token = $1"
        )
        .bind(token)
        .fetch_optional(pool)
        .await?;
        Ok(user)
    }

    pub async fn update_watch_history(pool: &DbPool, email: &str, watched: Vec<String>) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET watched = $1 WHERE email = $2")
            .bind(&watched)
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn mark_watched(pool: &DbPool, email: &str, video_id: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET watched = array_remove(array_append(watched, $1), $1) WHERE email = $2")
            .bind(video_id)
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn mark_unwatched(pool: &DbPool, email: &str, video_id: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET watched = array_remove(watched, $1) WHERE email = $2")
            .bind(video_id)
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn clear_watch_history(pool: &DbPool, email: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET watched = '{}' WHERE email = $1")
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_subscriptions(pool: &DbPool, email: &str, subscriptions: Vec<String>) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET feed_needs_update = true, subscriptions = $1 WHERE email = $2")
            .bind(&subscriptions)
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn subscribe_channel(pool: &DbPool, email: &str, channel_id: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET feed_needs_update = true, subscriptions = array_append(subscriptions, $1) WHERE email = $2")
            .bind(channel_id)
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn unsubscribe_channel(pool: &DbPool, email: &str, channel_id: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET feed_needs_update = true, subscriptions = array_remove(subscriptions, $1) WHERE email = $2")
            .bind(channel_id)
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn add_multiple_notifications(
        pool: &DbPool,
        channel_id: &str,
        video_ids: Vec<String>,
    ) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET notifications = array_cat(notifications, $1), feed_needs_update = true WHERE $2 = ANY(subscriptions)")
            .bind(&video_ids)
            .bind(channel_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn remove_notification(pool: &DbPool, email: &str, video_id: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET notifications = array_remove(notifications, $1) WHERE email = $2")
            .bind(video_id)
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn clear_notifications(pool: &DbPool, email: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET notifications = '{}', updated = now() WHERE email = $1")
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn feed_needs_update(pool: &DbPool, channel_id: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET feed_needs_update = true WHERE $1 = ANY(subscriptions)")
            .bind(channel_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_preferences(pool: &DbPool, email: &str, preferences: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET preferences = $1 WHERE email = $2")
            .bind(preferences)
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_password(pool: &DbPool, email: &str, password: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET password = $1 WHERE email = $2")
            .bind(password)
            .bind(email)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn select_notifications(pool: &DbPool, email: &str) -> anyhow::Result<Vec<String>> {
        let result = sqlx::query_as::<_, User>("SELECT notifications FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await?;
        
        match result {
            Some(user) => Ok(user.notifications.unwrap_or_default()),
            None => Ok(vec![]),
        }
    }
}
