//! Home page routes for Invidious.
//!
//! Provides static pages like home, privacy, licenses, and redirects.

use axum::{
    extract::Query,
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;

/// Query parameters for redirect endpoint.
#[derive(Debug, Deserialize)]
pub struct RedirectParams {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub instance: Option<String>,
}

/// Home page handler.
pub async fn home() -> Html<&'static str> {
    Html("<html><body><h1>Welcome to Invidious</h1></body></html>")
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
pub async fn redirect(Query(params): Query<RedirectParams>) -> impl IntoResponse {
    if let Some(path) = params.path {
        Redirect::to(&path)
    } else {
        Redirect::to("/")
    }
}
