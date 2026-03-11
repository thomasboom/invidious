//! Video watch routes for Invidious.
//!
//! Handles video playback, embeds, and related functionality.

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;

/// Query parameters for watch endpoint.
#[derive(Debug, Deserialize)]
pub struct WatchParams {
    #[serde(default)]
    pub v: Option<String>,
    #[serde(default)]
    pub list: Option<String>,
    #[serde(default)]
    pub index: Option<String>,
    #[serde(default)]
    pub t: Option<String>,
    #[serde(default)]
    pub local: Option<String>,
    #[serde(default)]
    pub autoplay: Option<String>,
    #[serde(default)]
    pub pbj: Option<String>,
    #[serde(default)]
    pub queue: Option<String>,
    #[serde(default)]
    pub loop_video: Option<String>,
    #[serde(default)]
    pub silent: Option<String>,
}

/// Main watch page handler.
pub async fn watch(Query(params): Query<WatchParams>) -> impl IntoResponse {
    if let Some(video_id) = params.v {
        Redirect::to(&format!("/watch/{}", video_id))
    } else {
        Redirect::to("/")
    }
}

/// Watch page with video ID.
pub async fn watch_id(Path(id): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Video: {}</h1></body></html>",
        id
    ))
}

/// Live video redirect handler.
pub async fn live_redirect(Path(id): Path<String>) -> Redirect {
    Redirect::to(&format!("/watch?v={}", id))
}

/// Shorts video redirect handler.
pub async fn shorts_redirect(Path(id): Path<String>) -> Redirect {
    Redirect::to(&format!("/watch?v={}", id))
}

/// Clip handler.
pub async fn clip(Path(clip_id): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Clip: {}</h1></body></html>",
        clip_id
    ))
}

/// Embed page handler.
pub async fn embed(Path(id): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Embed: {}</h1></body></html>",
        id
    ))
}

/// Embed redirect handler.
pub async fn embed_redirect() -> Redirect {
    Redirect::to("/")
}

/// Short watch path handler.
pub async fn watch_short(Path(id): Path<String>) -> Redirect {
    Redirect::to(&format!("/watch?v={}", id))
}

/// Video path handler.
pub async fn video_path(Path(id): Path<String>) -> Redirect {
    Redirect::to(&format!("/watch?v={}", id))
}

/// Embed path handler.
pub async fn embed_path(Path(id): Path<String>) -> Redirect {
    Redirect::to(&format!("/embed/{}", id))
}

/// Download handler for videos.
pub async fn download() -> Html<&'static str> {
    Html("<html><body><h1>Download</h1></body></html>")
}

/// Mark video as watched handler.
pub async fn mark_watched() -> Html<&'static str> {
    Html("{}")
}
