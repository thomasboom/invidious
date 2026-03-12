//! Feed routes for Invidious.
//!
//! Handles various feeds including trending, popular, subscriptions, and history.

use axum::{
    extract::{Path, Query, Extension},
    response::{Html, Redirect},
};
use serde::Deserialize;

use super::api::AppState;
use crate::templates::BaseTemplateData;

/// Query parameters for feed routes.
#[derive(Debug, Deserialize)]
pub struct FeedParams {
    #[serde(default)]
    pub page: Option<String>,
    #[serde(default)]
    pub continuation: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
}

/// View all playlists redirect handler.
pub async fn view_all_playlists() -> Redirect {
    Redirect::to("/feed/playlists")
}

/// Playlists feed handler.
pub async fn playlists() -> Html<&'static str> {
    Html("<html><body><h1>Your Playlists</h1></body></html>")
}

/// Popular feed handler.
pub async fn popular(Extension(state): Extension<AppState>) -> Html<String> {
    let base_data = BaseTemplateData {
        current_page: "/feed/popular".to_string(),
        ..Default::default()
    };
    
    let feed_context = serde_json::json!({
        "videos": [],
        "page_title": "Popular",
        "playlist_id": ""
    });
    
    match state.templates.render_with_data("popular.html", &feed_context) {
        Ok(content) => {
            match state.templates.render_base(&content, &base_data) {
                Ok(full) => Html(full),
                Err(_) => Html("<html><body>Error rendering template</body></html>".to_string()),
            }
        }
        Err(_) => Html("<html><body>Error loading template</body></html>".to_string()),
    }
}

/// Trending feed handler.
pub async fn trending(
    Extension(state): Extension<AppState>,
    Query(params): Query<FeedParams>,
) -> Html<String> {
    let trending_type = params.r#type.as_deref().unwrap_or("default");
    
    let base_data = BaseTemplateData {
        current_page: "/feed/trending".to_string(),
        ..Default::default()
    };
    
    let feed_context = serde_json::json!({
        "videos": [],
        "page_title": "Trending",
        "trending_type": trending_type,
        "playlist_id": ""
    });
    
    match state.templates.render_with_data("trending.html", &feed_context) {
        Ok(content) => {
            match state.templates.render_base(&content, &base_data) {
                Ok(full) => Html(full),
                Err(_) => Html("<html><body>Error rendering template</body></html>".to_string()),
            }
        }
        Err(_) => Html("<html><body>Error loading template</body></html>".to_string()),
    }
}

/// Subscriptions feed handler.
pub async fn subscriptions(Extension(state): Extension<AppState>) -> Html<String> {
    let base_data = BaseTemplateData {
        current_page: "/feed/subscriptions".to_string(),
        ..Default::default()
    };
    
    let feed_context = serde_json::json!({
        "videos": [],
        "playlist_id": ""
    });
    
    match state.templates.render_with_data("subscriptions.html", &feed_context) {
        Ok(content) => {
            match state.templates.render_base(&content, &base_data) {
                Ok(full) => Html(full),
                Err(_) => Html("<html><body>Error rendering template</body></html>".to_string()),
            }
        }
        Err(_) => Html("<html><body>Error loading template</body></html>".to_string()),
    }
}

/// Watch history feed handler.
pub async fn history() -> Html<&'static str> {
    Html("<html><body><h1>Watch History</h1></body></html>")
}

/// RSS feed for a channel.
pub async fn rss_channel(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<rss><channel><title>Channel {}</title></channel></rss>",
        ucid
    ))
}

/// RSS feed for private/user feed.
pub async fn rss_private() -> Html<&'static str> {
    Html("<rss><channel><title>Private Feed</title></channel></rss>")
}

/// RSS feed for a playlist.
pub async fn rss_playlist(Path(plid): Path<String>) -> Html<String> {
    Html(format!(
        "<rss><channel><title>Playlist {}</title></channel></rss>",
        plid
    ))
}

/// RSS videos feed handler.
pub async fn rss_videos() -> Html<&'static str> {
    Html("<rss><channel><title>Videos</title></channel></rss>")
}

/// Push notifications GET handler.
pub async fn push_notifications_get(Path(token): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Push Notifications: {}</h1></body></html>",
        token
    ))
}

/// Push notifications POST handler.
pub async fn push_notifications_post(Path(token): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Push Notifications POST: {}</h1></body></html>",
        token
    ))
}

/// Modify notifications handler.
pub async fn modify_notifications(Query(params): Query<FeedParams>) -> Html<String> {
    if let Some(continuation) = params.continuation {
        Html(format!(
            "<html><body><h1>Modify Notifications: {}</h1></body></html>",
            continuation
        ))
    } else {
        Html("<html><body><h1>Modify Notifications</h1></body></html>".to_string())
    }
}
