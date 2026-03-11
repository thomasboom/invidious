//! HTTP routes for Invidious.

use axum::{
    routing::{get},
    Router,
};

/// Create the main router for the application.
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/video/{id}", get(video))
        .route("/channel/{id}", get(channel))
        .route("/playlist/{id}", get(playlist))
        .route("/search", get(search))
        .route("/api/v1/videos/{id}", get(api_video))
        .route("/api/v1/search", get(api_search))
}

async fn home() -> &'static str {
    "Welcome to Invidious"
}

async fn video(axum::extract::Path(id): axum::extract::Path<String>) -> String {
    format!("Video: {}", id)
}

async fn channel(axum::extract::Path(id): axum::extract::Path<String>) -> String {
    format!("Channel: {}", id)
}

async fn playlist(axum::extract::Path(id): axum::extract::Path<String>) -> String {
    format!("Playlist: {}", id)
}

async fn search() -> &'static str {
    "Search"
}

async fn api_video(axum::extract::Path(id): axum::extract::Path<String>) -> String {
    format!("API Video: {}", id)
}

async fn api_search() -> &'static str {
    "API Search"
}
