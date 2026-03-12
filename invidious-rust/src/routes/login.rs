//! Login and authentication routes for Invidious.
//!
//! Handles user login, signout, and session management.

use axum::{
    extract::{Form, Query, Extension},
    response::Html,
};
use serde::Deserialize;

use super::api::AppState;
use crate::templates::BaseTemplateData;

/// Login form data.
#[derive(Debug, Deserialize)]
pub struct LoginForm {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub remember_me: Option<String>,
    #[serde(default)]
    pub captcha: Option<String>,
}

/// Query parameters for login.
#[derive(Debug, Deserialize)]
pub struct LoginParams {
    #[serde(default)]
    pub r: Option<String>,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub local: Option<String>,
}

/// Login page handler.
pub async fn login_page(
    Extension(state): Extension<AppState>,
    Query(params): Query<LoginParams>,
) -> Html<String> {
    let referer = params.r.as_deref().unwrap_or("/");
    
    let base_data = BaseTemplateData {
        current_page: "/login".to_string(),
        ..Default::default()
    };
    
    let login_context = serde_json::json!({
        "referer": referer,
        "csrf_token": "",
        "error": ""
    });
    
    match state.templates.render_with_data("login.html", &login_context) {
        Ok(content) => {
            match state.templates.render_base(&content, &base_data) {
                Ok(full) => Html(full),
                Err(_) => Html("<html><body>Error rendering template</body></html>".to_string()),
            }
        }
        Err(_) => Html("<html><body>Error loading template</body></html>".to_string()),
    }
}

/// Login handler.
pub async fn login(Form(_form): Form<LoginForm>) -> Html<&'static str> {
    Html("<html><body><h1>Login submitted</h1></body></html>")
}

/// Signout handler.
pub async fn signout(Query(params): Query<LoginParams>) -> Html<String> {
    let referer = params.r.as_deref().unwrap_or("/");
    Html(format!(
        "<html><body><h1>Signed out</h1><p>Referer: {}</p></body></html>",
        referer
    ))
}
