//! Feed routes for Invidious.
//!
//! Handles various feeds including trending, popular, subscriptions, and history.

use axum::{
    extract::{Path, Query},
    response::{Html, Redirect},
};
use serde::Deserialize;

/// Query parameters for feed routes.
#[derive(Debug, Deserialize)]
pub struct FeedParams {
    #[serde(default)]
    pub page: Option<String>,
    #[serde(default)]
    pub continuation: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
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
pub async fn popular() -> Html<&'static str> {
    Html("<html><body><h1>Popular Videos</h1></body></html>")
}

/// Trending feed handler.
pub async fn trending(Query(params): Query<FeedParams>) -> Html<String> {
    let category = params.category.as_deref().unwrap_or("music");
    Html(format!(
        "<html><body><h1>Trending: {}</h1></body></html>",
        category
    ))
}

/// Subscriptions feed handler.
pub async fn subscriptions() -> Html<&'static str> {
    Html("<html><body><h1>Your Subscriptions</h1></body></html>")
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
