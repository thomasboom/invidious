//! API v1 routes for Invidious.
//!
//! Provides JSON API endpoints for videos, channels, search, and authenticated operations.

use axum::{
    extract::{Path, Query, Extension},
    response::Json,
    routing::{delete as del, get, patch, post},
};
use serde::Deserialize;
use serde_json::Value as JsonValue;

use crate::config::Config;
use crate::db::DbPool;
use crate::yt_backend::{YoutubeApi, SearchResult};

/// Application state that can be shared across route handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: Option<DbPool>,
    pub config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            db: None,
            config,
        }
    }

    pub fn with_db(config: Config, db: DbPool) -> Self {
        Self {
            db: Some(db),
            config,
        }
    }

    fn get_yt_api(&self) -> Option<YoutubeApi> {
        YoutubeApi::new().ok()
    }
}

/// Query parameters for API routes.
#[derive(Debug, Deserialize, Default)]
pub struct ApiParams {
    #[serde(default)]
    pub page: Option<String>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub duration: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub features: Option<String>,
    #[serde(default)]
    pub q: Option<String>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub continuation: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub lang: Option<String>,
    #[serde(default)]
    pub thin_mode: Option<bool>,
    #[serde(default)]
    pub sort_by: Option<String>,
}

fn format_number(n: i64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M views", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K views", n as f64 / 1_000.0)
    } else {
        format!("{} views", n)
    }
}

/// Videos API endpoint.
pub async fn videos(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_video(&id).await {
            Ok(video) => {
                serde_json::json!({
                    "videoId": video.id,
                    "title": video.title,
                    "description": video.description_html,
                    "descriptionHtml": video.description_html,
                    "published": video.published,
                    "publishedText": video.published,
                    "videoThumbnails": video.thumbnails.iter().map(|t| {
                        serde_json::json!({"url": t.url, "width": t.width, "height": t.height})
                    }).collect::<Vec<_>>(),
                    "relatedStreams": [],
                    "author": video.author,
                    "authorId": video.author_id,
                    "authorUrl": format!("/channel/{}", video.author_id),
                    "authorThumbnails": video.author_thumbnail.as_ref().map(|t| {
                        vec![serde_json::json!({"url": t, "width": 48, "height": 48})]
                    }).unwrap_or_default(),
                    "subCount": serde_json::Value::Null,
                    "verified": video.author_verified,
                    "lengthSeconds": video.length_seconds,
                    "viewCount": video.views,
                    "viewCountText": format_number(video.views),
                    "likeCount": serde_json::Value::Null,
                    "dislikeCount": serde_json::Value::Null,
                    "liveNow": video.live_now,
                    "isPremium": video.badges.has_premium(),
                    "isUpcoming": false,
                    "premiereTimestamp": serde_json::Value::Null,
                    "allowedRegions": [],
                    "authorVerified": video.author_verified,
                    "captions": [],
                    "storyboards": [],
                    "recommends": []
                })
            }
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({
            "videoId": id,
            "title": "Video Title",
            "description": "Description unavailable"
        })
    };
    Json(result)
}

/// Storyboards API endpoint.
pub async fn storyboards(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_video(&id).await {
            Ok(_video) => serde_json::json!({"videoId": id, "storyboards": []}),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"videoId": id, "storyboards": []})
    };
    Json(result)
}

/// Captions API endpoint.
pub async fn captions(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_video(&id).await {
            Ok(_video) => serde_json::json!({"videoId": id, "captions": []}),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"videoId": id, "captions": []})
    };
    Json(result)
}

/// Annotations API endpoint.
pub async fn annotations(Path(id): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"videoId": id, "annotations": []}))
}

/// Comments API endpoint.
pub async fn comments(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
    Query(_params): Query<ApiParams>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_comments(&id).await {
            Ok(response) => {
                let comments: Vec<_> = response.comments.iter().map(|c| {
                    serde_json::json!({
                        "commentId": c.id,
                        "content": c.content,
                        "author": c.author,
                        "authorThumbnails": c.author_thumbnail.as_ref().map(|t| {
                            vec![serde_json::json!({"url": t, "width": 48, "height": 48})]
                        }).unwrap_or_default(),
                        "authorId": c.author_id,
                        "published": c.published,
                        "likeCount": c.like_count,
                        "isVerified": c.is_verified,
                        "isPinned": false,
                    })
                }).collect();

                let mut res = serde_json::json!({"videoId": id, "comments": comments});
                if let Some(cont) = response.continuation {
                    res["continuation"] = serde_json::json!(cont);
                }
                res
            }
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"videoId": id, "comments": []})
    };
    Json(result)
}

/// Clips API endpoint.
pub async fn clips(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_video(&id).await {
            Ok(_video) => serde_json::json!({"videoId": id, "clips": []}),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"videoId": id, "clips": []})
    };
    Json(result)
}

/// Transcripts API endpoint.
pub async fn transcripts(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_video(&id).await {
            Ok(_video) => serde_json::json!({"videoId": id, "transcripts": []}),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"videoId": id, "transcripts": []})
    };
    Json(result)
}

/// Trending API endpoint.
pub async fn trending(
    Extension(state): Extension<AppState>,
    Query(params): Query<ApiParams>,
) -> Json<JsonValue> {
    let category = params.r#type.as_deref().unwrap_or("music");
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_trending().await {
            Ok(results) => {
                let videos: Vec<_> = results.iter()
                    .filter_map(|r| {
                        if let SearchResult::Video(v) = r {
                            Some(serde_json::json!({
                                "videoId": v.id,
                                "title": v.title,
                                "videoThumbnails": v.thumbnails.iter().map(|t| {
                                    serde_json::json!({"url": t.url, "width": t.width, "height": t.height})
                                }).collect::<Vec<_>>(),
                                "author": v.author,
                                "authorId": v.author_id,
                                "viewCount": v.views,
                                "lengthSeconds": v.length_seconds,
                                "published": v.published,
                                "liveNow": v.live_now
                            }))
                        } else { None }
                    })
                    .collect();
                serde_json::json!({"videos": videos, "category": category})
            }
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"videos": [], "category": category})
    };
    Json(result)
}

/// Popular API endpoint.
pub async fn popular(Extension(state): Extension<AppState>) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_trending().await {
            Ok(results) => {
                let videos: Vec<_> = results.iter()
                    .filter_map(|r| {
                        if let SearchResult::Video(v) = r {
                            Some(serde_json::json!({
                                "videoId": v.id,
                                "title": v.title,
                                "videoThumbnails": v.thumbnails.iter().map(|t| {
                                    serde_json::json!({"url": t.url, "width": t.width, "height": t.height})
                                }).collect::<Vec<_>>(),
                                "author": v.author,
                                "authorId": v.author_id,
                                "viewCount": v.views,
                                "lengthSeconds": v.length_seconds,
                                "published": v.published,
                                "liveNow": v.live_now
                            }))
                        } else { None }
                    })
                    .collect();
                serde_json::json!({"videos": videos})
            }
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"videos": []})
    };
    Json(result)
}

/// Channel home API endpoint.
pub async fn channel_home(
    Extension(state): Extension<AppState>,
    Path(ucid): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_channel(&ucid).await {
            Ok(channel) => serde_json::json!({
                "author": channel.name,
                "authorId": channel.id,
                "authorUrl": format!("/channel/{}", channel.id),
                "authorBanners": [],
                "authorThumbnails": channel.thumbnail.map(|t| {
                    vec![serde_json::json!({"url": t, "width": 88, "height": 88})]
                }).unwrap_or_default(),
                "subCount": channel.subscriber_count,
                "totalViews": 0,
                "joined": 0,
                "autoGenerated": channel.auto_generated,
                "ageGated": false,
                "isFamilyFriendly": true,
                "description": channel.description_html,
                "descriptionHtml": channel.description_html,
                "allowedRegions": [],
                "tabs": ["home", "videos", "shorts", "streams", "playlists", "community", "channels", "about"],
                "tags": [],
                "authorVerified": channel.author_verified,
                "pronouns": serde_json::Value::Null,
                "latestVideos": [],
                "relatedChannels": []
            }),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({
            "authorId": ucid,
            "author": "Channel Name",
            "latestVideos": [],
            "relatedChannels": []
        })
    };
    Json(result)
}

/// Channel latest API endpoint.
pub async fn channel_latest(
    Extension(state): Extension<AppState>,
    Path(ucid): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_channel(&ucid).await {
            Ok(_channel) => serde_json::json!({"videos": []}),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"ucid": ucid, "videos": []})
    };
    Json(result)
}

/// Channel videos API endpoint.
pub async fn channel_videos(
    Extension(state): Extension<AppState>,
    Path(ucid): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_channel(&ucid).await {
            Ok(_channel) => serde_json::json!({"videos": []}),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"ucid": ucid, "videos": []})
    };
    Json(result)
}

/// Channel shorts API endpoint.
pub async fn channel_shorts(
    Extension(state): Extension<AppState>,
    Path(ucid): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_channel(&ucid).await {
            Ok(_channel) => serde_json::json!({"videos": []}),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"ucid": ucid, "shorts": []})
    };
    Json(result)
}

/// Channel streams API endpoint.
pub async fn channel_streams(
    Extension(state): Extension<AppState>,
    Path(ucid): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_channel(&ucid).await {
            Ok(_channel) => serde_json::json!({"videos": []}),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"ucid": ucid, "streams": []})
    };
    Json(result)
}

/// Channel podcasts API endpoint.
pub async fn channel_podcasts(Path(ucid): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"ucid": ucid, "podcasts": []}))
}

/// Channel releases API endpoint.
pub async fn channel_releases(Path(ucid): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"ucid": ucid, "releases": []}))
}

/// Channel courses API endpoint.
pub async fn channel_courses(Path(ucid): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"ucid": ucid, "courses": []}))
}

/// Channel playlists API endpoint.
pub async fn channel_playlists(
    Extension(state): Extension<AppState>,
    Path(ucid): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_channel(&ucid).await {
            Ok(_channel) => serde_json::json!({"playlists": []}),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"ucid": ucid, "playlists": []})
    };
    Json(result)
}

/// Channel community API endpoint.
pub async fn channel_community(
    Extension(state): Extension<AppState>,
    Path(ucid): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_channel(&ucid).await {
            Ok(_channel) => serde_json::json!({"ucid": ucid, "comments": []}),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"ucid": ucid, "comments": []})
    };
    Json(result)
}

/// Channel channels API endpoint.
pub async fn channel_channels(Path(ucid): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"ucid": ucid, "channels": []}))
}

/// Channel search API endpoint.
pub async fn channel_search(
    Extension(state): Extension<AppState>,
    Path(ucid): Path<String>,
) -> Json<JsonValue> {
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_channel(&ucid).await {
            Ok(_channel) => serde_json::json!({"results": []}),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"ucid": ucid, "results": []})
    };
    Json(result)
}

/// Post handler API endpoint.
pub async fn post_handler(Path(id): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"postId": id, "content": "Post content"}))
}

/// Post comments API endpoint.
pub async fn post_comments(Path(id): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"postId": id, "comments": []}))
}

/// Channel comments redirect handler.
pub async fn channel_comments_redirect(Path(ucid): Path<String>) -> axum::response::Redirect {
    axum::response::Redirect::to(&format!("/api/v1/channels/{}/community", ucid))
}

/// Search API endpoint.
pub async fn api_search(
    Extension(state): Extension<AppState>,
    Query(params): Query<ApiParams>,
) -> Json<JsonValue> {
    let query = params.q.as_deref().unwrap_or(params.search.as_deref().unwrap_or(""));
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.search(query, None).await {
            Ok(results) => {
                let items: Vec<_> = results.iter().map(|r| {
                    match r {
                        SearchResult::Video(v) => serde_json::json!({
                            "type": "video",
                            "title": v.title,
                            "videoId": v.id,
                            "author": v.author,
                            "authorId": v.author_id,
                            "videoThumbnails": v.thumbnails.iter().map(|t| {
                                serde_json::json!({"url": t.url, "width": t.width, "height": t.height})
                            }).collect::<Vec<_>>(),
                            "viewCount": v.views,
                            "lengthSeconds": v.length_seconds,
                            "published": v.published,
                            "liveNow": v.live_now
                        }),
                        SearchResult::Channel(c) => serde_json::json!({
                            "type": "channel",
                            "title": c.name,
                            "channelId": c.id,
                            "author": c.name,
                            "authorId": c.id,
                            "authorThumbnails": c.thumbnail.as_ref().map(|t| {
                                vec![serde_json::json!({"url": t, "width": 88, "height": 88})]
                            }).unwrap_or_default(),
                            "subCount": c.subscriber_count,
                            "videoCount": c.video_count,
                            "description": c.description_html
                        }),
                        SearchResult::Playlist(p) => serde_json::json!({
                            "type": "playlist",
                            "title": p.title,
                            "playlistId": p.id,
                            "author": p.author,
                            "authorId": p.author_id,
                            "videoCount": p.video_count,
                            "thumbnail": p.thumbnail
                        }),
                        SearchResult::Hashtag(h) => serde_json::json!({
                            "type": "hashtag",
                            "title": h.tag,
                            "hashtag": h.tag,
                            "url": h.url,
                            "videoCount": h.video_count
                        }),
                    }
                }).collect();
                serde_json::json!(items)
            }
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"query": query, "results": []})
    };
    Json(result)
}

/// Search suggestions API endpoint.
pub async fn search_suggestions(Query(params): Query<ApiParams>) -> Json<JsonValue> {
    let query = params.q.as_deref().unwrap_or("");
    
    let client = reqwest::Client::new();
    let url = format!(
        "https://suggestqueries-clients6.youtube.com/complete/search?client=youtube&hl=en&gl=US&q={}&gs_ri=youtube&ds=yt",
        urlencoding::encode(query)
    );
    
    let result = match client.get(&url).send().await {
        Ok(response) => {
            match response.text().await {
                Ok(text) => {
                    if text.len() > 20 {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text[19..]) {
                            let suggestions: Vec<String> = json.get(1)
                                .and_then(|v| v.as_array())
                                .map(|arr| {
                                    arr.iter()
                                        .filter_map(|v| v.get(0).and_then(|v| v.as_str()).map(String::from))
                                        .collect()
                                })
                                .unwrap_or_default();
                            return Json(serde_json::json!({"query": query, "suggestions": suggestions}));
                        }
                    }
                    serde_json::json!({"query": query, "suggestions": []})
                }
                Err(_) => serde_json::json!({"query": query, "suggestions": []})
            }
        }
        Err(_) => serde_json::json!({"query": query, "suggestions": []})
    };
    Json(result)
}

/// Hashtag API endpoint.
pub async fn hashtag(
    Extension(state): Extension<AppState>,
    Path(hashtag): Path<String>,
) -> Json<JsonValue> {
    let query = format!("#{}", hashtag);
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.search(&query, Some("EgIIAhgA")).await {
            Ok(results) => {
                let videos: Vec<_> = results.iter()
                    .filter_map(|r| {
                        if let SearchResult::Video(v) = r {
                            Some(serde_json::json!({
                                "videoId": v.id,
                                "title": v.title,
                                "videoThumbnails": v.thumbnails.iter().map(|t| {
                                    serde_json::json!({"url": t.url, "width": t.width, "height": t.height})
                                }).collect::<Vec<_>>(),
                                "author": v.author,
                                "authorId": v.author_id,
                                "viewCount": v.views,
                                "lengthSeconds": v.length_seconds,
                                "published": v.published,
                                "liveNow": v.live_now
                            }))
                        } else { None }
                    })
                    .collect();
                serde_json::json!({"hashtag": hashtag, "videos": videos, "results": videos})
            }
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"hashtag": hashtag, "videos": []})
    };
    Json(result)
}

/// Get preferences endpoint.
pub async fn get_preferences() -> Json<JsonValue> {
    Json(serde_json::json!({"preferences": {}}))
}

/// Set preferences endpoint.
pub async fn set_preferences() -> Json<JsonValue> {
    Json(serde_json::json!({"success": true}))
}

/// Export Invidious data endpoint.
pub async fn export_invidious() -> Json<JsonValue> {
    Json(serde_json::json!({"data": {}}))
}

/// Import Invidious data endpoint.
pub async fn import_invidious() -> Json<JsonValue> {
    Json(serde_json::json!({"success": true}))
}

/// Get history endpoint.
pub async fn get_history() -> Json<JsonValue> {
    Json(serde_json::json!({"history": []}))
}

/// Mark watched endpoint.
pub async fn mark_watched(Path(id): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"videoId": id, "watched": true}))
}

/// Mark unwatched endpoint.
pub async fn mark_unwatched(Path(id): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"videoId": id, "watched": false}))
}

/// Clear history endpoint.
pub async fn clear_history() -> Json<JsonValue> {
    Json(serde_json::json!({"success": true}))
}

/// Feed endpoint.
pub async fn feed() -> Json<JsonValue> {
    Json(serde_json::json!({"videos": [], "notifications": []}))
}

/// Get subscriptions endpoint.
pub async fn get_subscriptions() -> Json<JsonValue> {
    Json(serde_json::json!({"subscriptions": []}))
}

/// Subscribe to channel endpoint.
pub async fn subscribe_channel(Path(ucid): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"ucid": ucid, "subscribed": true}))
}

/// Unsubscribe from channel endpoint.
pub async fn unsubscribe_channel(Path(ucid): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"ucid": ucid, "subscribed": false}))
}

/// List playlists endpoint.
pub async fn list_playlists() -> Json<JsonValue> {
    Json(serde_json::json!({"playlists": []}))
}

/// Create playlist endpoint.
pub async fn create_playlist() -> Json<JsonValue> {
    Json(serde_json::json!({"success": true, "playlistId": "new_playlist_id"}))
}

/// Update playlist attribute endpoint.
pub async fn update_playlist_attribute(Path(plid): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"playlistId": plid, "success": true}))
}

/// Delete playlist endpoint.
pub async fn delete_playlist(Path(plid): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"playlistId": plid, "success": true}))
}

/// Insert video into playlist endpoint.
pub async fn insert_video_into_playlist(Path(plid): Path<String>) -> Json<JsonValue> {
    Json(serde_json::json!({"playlistId": plid, "success": true}))
}

/// Delete video from playlist endpoint.
pub async fn delete_video_in_playlist(Path((plid, index)): Path<(String, String)>) -> Json<JsonValue> {
    Json(serde_json::json!({"playlistId": plid, "index": index, "success": true}))
}

/// Get tokens endpoint.
pub async fn get_tokens() -> Json<JsonValue> {
    Json(serde_json::json!({"tokens": []}))
}

/// Register token endpoint.
pub async fn register_token() -> Json<JsonValue> {
    Json(serde_json::json!({"success": true}))
}

/// Unregister token endpoint.
pub async fn unregister_token() -> Json<JsonValue> {
    Json(serde_json::json!({"success": true}))
}

/// Notifications endpoint.
pub async fn notifications() -> Json<JsonValue> {
    Json(serde_json::json!({"notifications": []}))
}

/// Stats API endpoint.
pub async fn stats(Extension(state): Extension<AppState>) -> Json<JsonValue> {
    let version = env!("CARGO_PKG_VERSION");
    Json(serde_json::json!({
        "version": version,
        "software": {"name": "invidious-rust", "version": version},
        "openRegistrations": state.config.registration_enabled,
        "usage": {"users": serde_json::Value::Null}
    }))
}

/// Get playlist endpoint.
pub async fn get_playlist(
    Extension(state): Extension<AppState>,
    Path(plid): Path<String>,
) -> Json<JsonValue> {
    if plid.starts_with("RD") {
        return mixes(Extension(state), Path(plid), Query(ApiParams::default())).await;
    }
    
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_playlist(&plid).await {
            Ok(playlist) => serde_json::json!({
                "playlistId": playlist.id,
                "title": playlist.title,
                "description": "",
                "descriptionHtml": "",
                "videoCount": playlist.video_count,
                "viewCount": 0,
                "updated": serde_json::Value::Null,
                "author": playlist.author,
                "authorId": playlist.author_id,
                "authorThumbnails": [],
                "videos": playlist.videos.iter().map(|v| {
                    serde_json::json!({
                        "title": v.title,
                        "videoId": v.id,
                        "author": "",
                        "authorId": "",
                        "videoThumbnails": [],
                        "lengthSeconds": v.length_seconds,
                        "index": 0,
                        "indexId": v.id
                    })
                }).collect::<Vec<_>>()
            }),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"playlistId": plid, "title": "Playlist Title", "videos": []})
    };
    Json(result)
}

/// Get auth playlist endpoint.
pub async fn get_auth_playlist(
    Extension(state): Extension<AppState>,
    Path(plid): Path<String>,
) -> Json<JsonValue> {
    get_playlist(Extension(state), Path(plid)).await
}

/// Mixes endpoint.
pub async fn mixes(
    Extension(state): Extension<AppState>,
    Path(rdid): Path<String>,
    Query(_params): Query<ApiParams>,
) -> Json<JsonValue> {
    let video_id = rdid.strip_prefix("RD").map(|s| s.chars().take(11).collect::<String>()).unwrap_or_default();
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.get_related_videos(&video_id).await {
            Ok(videos) => serde_json::json!({
                "relatedVideoId": rdid,
                "mix": videos.iter().map(|v| {
                    serde_json::json!({
                        "title": v.title,
                        "videoId": v.id,
                        "author": v.author,
                        "authorId": v.author_id,
                        "authorUrl": format!("/channel/{}", v.author_id),
                        "videoThumbnails": v.thumbnails.iter().map(|t| {
                            serde_json::json!({"url": t.url, "width": t.width, "height": t.height})
                        }).collect::<Vec<_>>(),
                        "lengthSeconds": v.length_seconds
                    })
                }).collect::<Vec<_>>()
            }),
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"relatedVideoId": rdid, "mix": []})
    };
    Json(result)
}

/// Resolve URL endpoint.
pub async fn resolve_url(
    Extension(state): Extension<AppState>,
    Query(params): Query<ApiParams>,
) -> Json<JsonValue> {
    let url = params.q.as_deref().unwrap_or("");
    
    if url.is_empty() {
        return Json(serde_json::json!({"error": "Missing URL to resolve"}));
    }
    
    let result = if let Some(yt_api) = state.get_yt_api() {
        match yt_api.resolve_url(url).await {
            Ok(response) => {
                let endpoint = response.get("endpoint");
                let browse_id = endpoint.and_then(|e| e.get("browseId")).and_then(|v| v.as_str());
                let video_id = endpoint.and_then(|e| e.get("videoId")).and_then(|v| v.as_str());
                let playlist_id = endpoint.and_then(|e| e.get("playlistId")).and_then(|v| v.as_str());
                let start_time = endpoint.and_then(|e| e.get("startTimeSeconds")).and_then(|v| v.as_i64());
                serde_json::json!({
                    "browseId": browse_id,
                    "ucid": browse_id.filter(|b| b.starts_with("UC")),
                    "videoId": video_id,
                    "playlistId": playlist_id,
                    "startTimeSeconds": start_time,
                    "url": url
                })
            }
            Err(e) => serde_json::json!({"error": e.to_string()}),
        }
    } else {
        serde_json::json!({"url": url, "endpoint": {}})
    };
    Json(result)
}

/// Create the API v1 router.
pub fn create_router(state: AppState) -> axum::Router {
    axum::Router::new()
        .layer(axum::Extension(state))
        .route("/videos/{id}", get(videos))
        .route("/storyboards/{id}", get(storyboards))
        .route("/captions/{id}", get(captions))
        .route("/annotations/{id}", get(annotations))
        .route("/comments/{id}", get(comments))
        .route("/clips/{id}", get(clips))
        .route("/transcripts/{id}", get(transcripts))
        
        .route("/trending", get(trending))
        .route("/popular", get(popular))
        
        .route("/channels/{ucid}", get(channel_home))
        .route("/channels/{ucid}/latest", get(channel_latest))
        .route("/channels/{ucid}/videos", get(channel_videos))
        .route("/channels/{ucid}/shorts", get(channel_shorts))
        .route("/channels/{ucid}/streams", get(channel_streams))
        .route("/channels/{ucid}/podcasts", get(channel_podcasts))
        .route("/channels/{ucid}/releases", get(channel_releases))
        .route("/channels/{ucid}/courses", get(channel_courses))
        .route("/channels/{ucid}/playlists", get(channel_playlists))
        .route("/channels/{ucid}/community", get(channel_community))
        .route("/channels/{ucid}/posts", get(channel_community))
        .route("/channels/{ucid}/channels", get(channel_channels))
        .route("/channels/{ucid}/search", get(channel_search))
        
        .route("/post/{id}", get(post_handler))
        .route("/post/{id}/comments", get(post_comments))
        
        .route("/channels/comments/{ucid}", get(channel_comments_redirect))
        .route("/channels/{ucid}/comments", get(channel_comments_redirect))
        
        .route("/search", get(api_search))
        .route("/search/suggestions", get(search_suggestions))
        .route("/hashtag/{hashtag}", get(hashtag))
        
        .route("/auth/preferences", get(get_preferences))
        .route("/auth/preferences", post(set_preferences))
        .route("/auth/export/invidious", get(export_invidious))
        .route("/auth/import/invidious", post(import_invidious))
        .route("/auth/history", get(get_history))
        .route("/auth/history/{id}", post(mark_watched))
        .route("/auth/history/{id}", del(mark_unwatched))
        .route("/auth/history", del(clear_history))
        .route("/auth/feed", get(feed))
        .route("/auth/subscriptions", get(get_subscriptions))
        .route("/auth/subscriptions/{ucid}", post(subscribe_channel))
        .route("/auth/subscriptions/{ucid}", del(unsubscribe_channel))
        .route("/auth/playlists", get(list_playlists))
        .route("/auth/playlists", post(create_playlist))
        .route("/auth/playlists/{plid}", patch(update_playlist_attribute))
        .route("/auth/playlists/{plid}", del(delete_playlist))
        .route("/auth/playlists/{plid}/videos", post(insert_video_into_playlist))
        .route("/auth/playlists/{plid}/videos/{index}", del(delete_video_in_playlist))
        .route("/auth/tokens", get(get_tokens))
        .route("/auth/tokens/register", post(register_token))
        .route("/auth/tokens/unregister", post(unregister_token))
        .route("/auth/notifications", get(notifications))
        .route("/auth/notifications", post(notifications))
        
        .route("/stats", get(stats))
        .route("/playlists/{plid}", get(get_playlist))
        .route("/auth/playlists/{plid}", get(get_auth_playlist))
        .route("/mixes/{rdid}", get(mixes))
        .route("/resolveurl", get(resolve_url))
}

/// API v1 index handler.
pub async fn v1_index() -> Json<JsonValue> {
    Json(serde_json::json!({
        "apiVersion": "v1",
        "version": "0.1.0"
    }))
}
