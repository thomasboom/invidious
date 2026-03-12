//! Home page routes for Invidious.
//!
//! Provides static pages like home, privacy, licenses, and redirects.

use axum::{
    extract::{Query, Extension},
    response::Html,
};
use serde::Deserialize;

use super::api::AppState;
use crate::templates::BaseTemplateData;

/// Query parameters for redirect endpoint.
#[derive(Debug, Deserialize)]
pub struct RedirectParams {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub instance: Option<String>,
}

/// Home page handler.
pub async fn home(Extension(state): Extension<AppState>) -> Html<String> {
    let base_data = BaseTemplateData {
        current_page: "/".to_string(),
        ..Default::default()
    };
    
    let home_context = serde_json::json!({
        "videos": [],
        "page_title": "Popular",
        "playlist_id": ""
    });
    
    match state.templates.render_with_data("home.html", &home_context) {
        Ok(content) => {
            match state.templates.render_base(&content, &base_data) {
                Ok(full) => Html(full),
                Err(_) => Html("<html><body>Error rendering template</body></html>".to_string()),
            }
        }
        Err(_) => Html("<html><body>Error loading template</body></html>".to_string()),
    }
}

/// Privacy policy page handler.
pub async fn privacy() -> Html<&'static str> {
    Html("<html><body><h1>Privacy Policy</h1></body></html>")
}

/// Licenses page handler.
pub async fn licenses() -> Html<&'static str> {
    Html("<html><body><h1>Open Source Licenses</h1></body></html>")
}

/// Cross-instance redirect handler.
pub async fn redirect(Query(params): Query<RedirectParams>) -> impl axum::response::IntoResponse {
    if let Some(path) = params.path {
        axum::response::Redirect::to(&path)
    } else {
        axum::response::Redirect::to("/")
    }
}
