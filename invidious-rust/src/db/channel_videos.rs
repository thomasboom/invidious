//! Channel video database operations.

use crate::db::DbPool;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, Row};

#[derive(Debug, Clone, FromRow)]
pub struct ChannelVideo {
    pub id: String,
    pub title: Option<String>,
    pub published: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub ucid: Option<String>,
    pub author: Option<String>,
    pub length_seconds: Option<i32>,
    pub live_now: Option<bool>,
    pub premiere_timestamp: Option<DateTime<Utc>>,
    pub views: Option<i64>,
}

pub struct ChannelVideos;

impl ChannelVideos {
    pub async fn insert(
        pool: &DbPool,
        video: &ChannelVideo,
        with_premiere_timestamp: bool,
    ) -> anyhow::Result<bool> {
        let last_items = if with_premiere_timestamp {
            "premiere_timestamp = $9, views = $10"
        } else {
            "views = $9"
        };

        let query_string = format!(
            r#"
            INSERT INTO channel_videos
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE
            SET title = $2, published = $3, updated = $4, ucid = $5,
                author = $6, length_seconds = $7, live_now = $8, {}
            RETURNING (xmax=0) AS was_insert
            "#,
            last_items
        );

        let query = sqlx::query(&query_string);

        let result = query
            .bind(&video.id)
            .bind(&video.title)
            .bind(&video.published)
            .bind(&video.updated)
            .bind(&video.ucid)
            .bind(&video.author)
            .bind(&video.length_seconds)
            .bind(&video.live_now)
            .bind(&video.premiere_timestamp)
            .bind(&video.views)
            .fetch_one(pool)
            .await?;

        let was_insert = result.get::<bool, _>("was_insert");
        Ok(was_insert)
    }

    pub async fn select_many(pool: &DbPool, ids: &[String]) -> anyhow::Result<Vec<ChannelVideo>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let videos = sqlx::query_as::<_, ChannelVideo>(
            "SELECT id, title, published, updated, ucid, author, length_seconds, live_now, premiere_timestamp, views FROM channel_videos WHERE id = ANY($1) ORDER BY published DESC"
        )
        .bind(ids)
        .fetch_all(pool)
        .await?;
        Ok(videos)
    }

    pub async fn select_notifications(
        pool: &DbPool,
        ucid: &str,
        since: DateTime<Utc>,
    ) -> anyhow::Result<Vec<ChannelVideo>> {
        let videos = sqlx::query_as::<_, ChannelVideo>(
            "SELECT id, title, published, updated, ucid, author, length_seconds, live_now, premiere_timestamp, views FROM channel_videos WHERE ucid = $1 AND published > $2 ORDER BY published DESC LIMIT 15"
        )
        .bind(ucid)
        .bind(since)
        .fetch_all(pool)
        .await?;
        Ok(videos)
    }

    pub async fn select_popular_videos(pool: &DbPool) -> anyhow::Result<Vec<ChannelVideo>> {
        let videos = sqlx::query_as::<_, ChannelVideo>(
            r#"
            SELECT DISTINCT ON (cv.ucid) cv.id, cv.title, cv.published, cv.updated, cv.ucid, cv.author, cv.length_seconds, cv.live_now, cv.premiere_timestamp, cv.views
            FROM channel_videos cv
            WHERE cv.ucid IN (SELECT channel FROM (SELECT UNNEST(subscriptions) AS channel FROM users) AS d GROUP BY channel ORDER BY COUNT(channel) DESC LIMIT 40)
            ORDER BY cv.ucid, cv.published DESC
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(videos)
    }
}
