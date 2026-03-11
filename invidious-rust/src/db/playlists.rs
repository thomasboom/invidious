//! Playlist database operations.

use crate::db::DbPool;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlaylistPrivacy {
    Public,
    Unlisted,
    Private,
}

impl PlaylistPrivacy {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Public" => PlaylistPrivacy::Public,
            "Unlisted" => PlaylistPrivacy::Unlisted,
            "Private" => PlaylistPrivacy::Private,
            _ => PlaylistPrivacy::Private,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PlaylistPrivacy::Public => "Public",
            PlaylistPrivacy::Unlisted => "Unlisted",
            PlaylistPrivacy::Private => "Private",
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct Playlist {
    pub title: Option<String>,
    pub id: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub video_count: Option<i32>,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub privacy: String,
    pub index: Option<Vec<i64>>,
}

impl Playlist {
    pub fn privacy_enum(&self) -> PlaylistPrivacy {
        PlaylistPrivacy::from_str(&self.privacy)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct PlaylistVideo {
    pub title: Option<String>,
    pub id: Option<String>,
    pub author: Option<String>,
    pub ucid: Option<String>,
    pub length_seconds: Option<i32>,
    pub published: Option<DateTime<Utc>>,
    pub plid: String,
    pub index: i64,
    pub live_now: Option<bool>,
}

pub struct Playlists;

impl Playlists {
    pub async fn insert(pool: &DbPool, playlist: &Playlist) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO playlists (title, id, author, description, video_count, created, updated, privacy, index) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(&playlist.title)
        .bind(&playlist.id)
        .bind(&playlist.author)
        .bind(&playlist.description)
        .bind(playlist.video_count)
        .bind(playlist.created)
        .bind(playlist.updated)
        .bind(&playlist.privacy)
        .bind(&playlist.index)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &DbPool, id: &str) -> anyhow::Result<()> {
        PlaylistVideos::delete_by_playlist(pool, id).await?;
        sqlx::query("DELETE FROM playlists WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update(
        pool: &DbPool,
        id: &str,
        title: &str,
        privacy: PlaylistPrivacy,
        description: Option<&str>,
    ) -> anyhow::Result<()> {
        sqlx::query("UPDATE playlists SET title = $1, privacy = $2, description = $3, updated = now() WHERE id = $4")
            .bind(title)
            .bind(privacy.as_str())
            .bind(description)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_description(pool: &DbPool, id: &str, description: Option<&str>) -> anyhow::Result<()> {
        sqlx::query("UPDATE playlists SET description = $1 WHERE id = $2")
            .bind(description)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_subscription_time(pool: &DbPool, id: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE playlists SET subscribed = now() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_video_added(pool: &DbPool, id: &str, index: i64) -> anyhow::Result<()> {
        sqlx::query("UPDATE playlists SET index = array_append(index, $1), video_count = cardinality(index) + 1, updated = now() WHERE id = $2")
            .bind(index)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_video_removed(pool: &DbPool, id: &str, index: i64) -> anyhow::Result<()> {
        sqlx::query("UPDATE playlists SET index = array_remove(index, $1), video_count = cardinality(index) - 1, updated = now() WHERE id = $2")
            .bind(index)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn select(pool: &DbPool, id: &str) -> anyhow::Result<Option<Playlist>> {
        let playlist = sqlx::query_as::<_, Playlist>(
            "SELECT title, id, author, description, video_count, created, updated, privacy, index FROM playlists WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(playlist)
    }

    pub async fn select_all_by_author(pool: &DbPool, author: &str) -> anyhow::Result<Vec<Playlist>> {
        let playlists = sqlx::query_as::<_, Playlist>(
            "SELECT title, id, author, description, video_count, created, updated, privacy, index FROM playlists WHERE author = $1"
        )
        .bind(author)
        .fetch_all(pool)
        .await?;
        Ok(playlists)
    }

    pub async fn select_like_iv(pool: &DbPool, email: &str) -> anyhow::Result<Vec<Playlist>> {
        let playlists = sqlx::query_as::<_, Playlist>(
            "SELECT title, id, author, description, video_count, created, updated, privacy, index FROM playlists WHERE author = $1 AND id LIKE 'IV%' ORDER BY created"
        )
        .bind(email)
        .fetch_all(pool)
        .await?;
        Ok(playlists)
    }

    pub async fn select_not_like_iv(pool: &DbPool, email: &str) -> anyhow::Result<Vec<Playlist>> {
        let playlists = sqlx::query_as::<_, Playlist>(
            "SELECT title, id, author, description, video_count, created, updated, privacy, index FROM playlists WHERE author = $1 AND id NOT LIKE 'IV%' ORDER BY created"
        )
        .bind(email)
        .fetch_all(pool)
        .await?;
        Ok(playlists)
    }

    pub async fn select_user_created_playlists(pool: &DbPool, email: &str) -> anyhow::Result<Vec<(String, String)>> {
        #[derive(sqlx::FromRow)]
        struct PlaylistTitle {
            id: String,
            title: String,
        }

        let playlists = sqlx::query_as::<_, PlaylistTitle>(
            "SELECT id, title FROM playlists WHERE author = $1 AND id LIKE 'IV%' ORDER BY title"
        )
        .bind(email)
        .fetch_all(pool)
        .await?;

        Ok(playlists.into_iter().map(|p| (p.id, p.title)).collect())
    }

    pub async fn exists(pool: &DbPool, id: &str) -> anyhow::Result<bool> {
        #[derive(sqlx::FromRow)]
        struct IdResult {
            id: String,
        }
        
        let result = sqlx::query_as::<_, IdResult>("SELECT id FROM playlists WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(result.is_some())
    }

    pub async fn count_owned_by(pool: &DbPool, author: &str) -> anyhow::Result<i64> {
        let row = sqlx::query("SELECT count(*) FROM playlists WHERE author = $1")
            .bind(author)
            .fetch_one(pool)
            .await?;
        Ok(row.get::<i64, _>(0))
    }
}

pub struct PlaylistVideos;

impl PlaylistVideos {
    pub async fn insert(pool: &DbPool, video: &PlaylistVideo) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO playlist_videos (title, id, author, ucid, length_seconds, published, plid, index, live_now) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(&video.title)
        .bind(&video.id)
        .bind(&video.author)
        .bind(&video.ucid)
        .bind(video.length_seconds)
        .bind(video.published)
        .bind(&video.plid)
        .bind(video.index)
        .bind(video.live_now)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &DbPool, index: i64) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM playlist_videos WHERE index = $1")
            .bind(index)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete_by_playlist(pool: &DbPool, plid: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM playlist_videos WHERE plid = $1")
            .bind(plid)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn select(
        pool: &DbPool,
        plid: &str,
        index: &[i64],
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<PlaylistVideo>> {
        let videos = sqlx::query_as::<_, PlaylistVideo>(
            "SELECT title, id, author, ucid, length_seconds, published, plid, index, live_now FROM playlist_videos WHERE plid = $1 ORDER BY array_position($2, index) LIMIT $3 OFFSET $4"
        )
        .bind(plid)
        .bind(index)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        Ok(videos)
    }

    pub async fn select_index(pool: &DbPool, plid: &str, vid: &str) -> anyhow::Result<Option<i64>> {
        #[derive(sqlx::FromRow)]
        struct IndexResult {
            index: i64,
        }
        
        let result = sqlx::query_as::<_, IndexResult>(
            "SELECT index FROM playlist_videos WHERE plid = $1 AND id = $2 LIMIT 1"
        )
        .bind(plid)
        .bind(vid)
        .fetch_optional(pool)
        .await?;
        
        Ok(result.map(|r| r.index))
    }

    pub async fn select_one_id(pool: &DbPool, plid: &str, index: &[i64]) -> anyhow::Result<Option<String>> {
        #[derive(sqlx::FromRow)]
        struct IdResult {
            id: String,
        }
        
        let result = sqlx::query_as::<_, IdResult>(
            "SELECT id FROM playlist_videos WHERE plid = $1 ORDER BY array_position($2, index) LIMIT 1"
        )
        .bind(plid)
        .bind(index)
        .fetch_optional(pool)
        .await?;
        
        Ok(result.map(|r| r.id))
    }

    pub async fn select_ids(pool: &DbPool, plid: &str, index: &[i64], limit: i64) -> anyhow::Result<Vec<String>> {
        #[derive(sqlx::FromRow)]
        struct IdResult {
            id: String,
        }
        
        let videos = sqlx::query_as::<_, IdResult>(
            "SELECT id FROM playlist_videos WHERE plid = $1 ORDER BY array_position($2, index) LIMIT $3"
        )
        .bind(plid)
        .bind(index)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(videos.into_iter().map(|v| v.id).collect())
    }
}
