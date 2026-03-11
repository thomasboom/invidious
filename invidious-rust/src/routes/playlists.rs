//! Playlist routes for Invidious.
//!
//! Handles playlist viewing, creation, editing, and management.

use axum::{
    extract::Query,
    response::Html,
};
use serde::Deserialize;

/// Query parameters for playlist routes.
#[derive(Debug, Deserialize)]
pub struct PlaylistParams {
    #[serde(default)]
    pub list: Option<String>,
    #[serde(default)]
    pub index: Option<String>,
    #[serde(default)]
    pub v: Option<String>,
    #[serde(default)]
    pub t: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub privacy: Option<String>,
    #[serde(default)]
    pub plid: Option<String>,
}

/// Show playlist handler.
pub async fn show_playlist(Query(params): Query<PlaylistParams>) -> Html<String> {
    if let Some(list) = params.list {
        Html(format!(
            "<html><body><h1>Playlist: {}</h1></body></html>",
            list
        ))
    } else {
        Html("<html><body><h1>Playlists</h1></body></html>".to_string())
    }
}

/// Create playlist page handler.
pub async fn create_playlist_page() -> Html<&'static str> {
    Html("<html><body><h1>Create Playlist</h1></body></html>")
}

/// Create playlist handler.
pub async fn create_playlist() -> Html<&'static str> {
    Html("{}")
}

/// Subscribe to playlist handler.
pub async fn subscribe_playlist(Query(params): Query<PlaylistParams>) -> Html<String> {
    if let Some(plid) = params.plid {
        Html(format!("<html><body><h1>Subscribed to: {}</h1></body></html>", plid))
    } else {
        Html("<html><body><h1>Subscribe to Playlist</h1></body></html>".to_string())
    }
}

/// Delete playlist page handler.
pub async fn delete_playlist_page(Query(params): Query<PlaylistParams>) -> Html<String> {
    if let Some(plid) = params.plid {
        Html(format!(
            "<html><body><h1>Delete Playlist: {}</h1></body></html>",
            plid
        ))
    } else {
        Html("<html><body><h1>Delete Playlist</h1></body></html>".to_string())
    }
}

/// Delete playlist handler.
pub async fn delete_playlist() -> Html<&'static str> {
    Html("{}")
}

/// Edit playlist page handler.
pub async fn edit_playlist(Query(params): Query<PlaylistParams>) -> Html<String> {
    if let Some(plid) = params.plid {
        Html(format!(
            "<html><body><h1>Edit Playlist: {}</h1></body></html>",
            plid
        ))
    } else {
        Html("<html><body><h1>Edit Playlist</h1></body></html>".to_string())
    }
}

/// Update playlist handler.
pub async fn update_playlist() -> Html<&'static str> {
    Html("{}")
}

/// Add playlist items page handler.
pub async fn add_playlist_items_page(Query(params): Query<PlaylistParams>) -> Html<String> {
    if let Some(plid) = params.plid {
        Html(format!(
            "<html><body><h1>Add Items to Playlist: {}</h1></body></html>",
            plid
        ))
    } else {
        Html("<html><body><h1>Add Playlist Items</h1></body></html>".to_string())
    }
}

/// Playlist AJAX handler.
pub async fn playlist_ajax() -> Html<&'static str> {
    Html("{}")
}

/// Mix playlist handler.
pub async fn mix(Query(params): Query<PlaylistParams>) -> Html<String> {
    if let Some(list) = params.list {
        Html(format!(
            "<html><body><h1>Mix: {}</h1></body></html>",
            list
        ))
    } else {
        Html("<html><body><h1>Mix</h1></body></html>".to_string())
    }
}

/// Watch videos handler.
pub async fn watch_videos(Query(params): Query<PlaylistParams>) -> Html<String> {
    if let Some(list) = params.list {
        Html(format!(
            "<html><body><h1>Watch Videos: {}</h1></body></html>",
            list
        ))
    } else {
        Html("<html><body><h1>Watch Videos</h1></body></html>".to_string())
    }
}
