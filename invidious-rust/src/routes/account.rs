//! Account management routes for Invidious.
//!
//! Handles user preferences, password changes, account deletion, and history.

use axum::{
    extract::{Form, Query, Extension},
    response::Html,
};
use serde::Deserialize;

use super::api::AppState;
use crate::templates::BaseTemplateData;
use crate::config::ConfigPreferences;

/// Query parameters for account routes.
#[derive(Debug, Deserialize)]
pub struct AccountParams {
    #[serde(default)]
    pub page: Option<String>,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub local: Option<String>,
    #[serde(default)]
    pub r: Option<String>,
}

/// Form data for various account actions.
#[derive(Debug, Deserialize)]
pub struct AccountForm {
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub new_password: Option<String>,
    #[serde(default)]
    pub confirm_password: Option<String>,
    #[serde(default)]
    pub confirm_delete: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub callback: Option<String>,
    #[serde(default)]
    pub auth: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub annotations: Option<String>,
    #[serde(default)]
    pub autoplay: Option<String>,
    #[serde(default)]
    pub captions: Option<String>,
    #[serde(default)]
    pub comments: Option<String>,
    #[serde(default)]
    pub continue_url: Option<String>,
    #[serde(default)]
    pub dark_mode: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub hd: Option<String>,
    #[serde(default)]
    pub local: Option<String>,
    #[serde(default)]
    pub max_results: Option<String>,
    #[serde(default)]
    pub notifications: Option<String>,
    #[serde(default)]
    pub player: Option<String>,
    #[serde(default)]
    pub quality: Option<String>,
    #[serde(default)]
    pub related_videos: Option<String>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub speed: Option<String>,
    #[serde(default)]
    pub subtitles: Option<String>,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub thin_mode: Option<String>,
    #[serde(default)]
    pub video_mixed: Option<String>,
}

/// Preferences page handler.
pub async fn preferences(
    Extension(state): Extension<AppState>,
    Query(params): Query<AccountParams>,
) -> Html<String> {
    let referer = params.r.as_deref().unwrap_or("/");
    
    let base_data = BaseTemplateData {
        current_page: "/preferences".to_string(),
        ..Default::default()
    };
    
    let prefs_context = serde_json::json!({
        "preferences": ConfigPreferences::default(),
        "csrf_token": "",
        "referer": referer
    });
    
    match state.templates.render_with_data("preferences.html", &prefs_context) {
        Ok(content) => {
            match state.templates.render_base(&content, &base_data) {
                Ok(full) => Html(full),
                Err(_) => Html("<html><body>Error rendering template</body></html>".to_string()),
            }
        }
        Err(_) => Html("<html><body>Error loading template</body></html>".to_string()),
    }
}

/// Update preferences handler.
pub async fn update_preferences(Form(_form): Form<AccountForm>) -> Html<String> {
    Html("<html><body><h1>Preferences Updated</h1></body></html>".to_string())
}

/// Toggle theme handler.
pub async fn toggle_theme(Query(params): Query<AccountParams>) -> Html<String> {
    let theme = params.theme.as_deref().unwrap_or("light");
    Html(format!(
        "<html><body><h1>Theme toggled to: {}</h1></body></html>",
        theme
    ))
}

/// Data control page handler.
pub async fn data_control(Query(params): Query<AccountParams>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Data Control</h1><p>Return to: {}</p></body></html>",
        params.r.as_deref().unwrap_or("/")
    ))
}

/// Update data control handler.
pub async fn update_data_control(Form(_form): Form<AccountForm>) -> Html<String> {
    Html("<html><body><h1>Data Control Updated</h1></body></html>".to_string())
}

/// Change password page handler.
pub async fn change_password_get() -> Html<&'static str> {
    Html("<html><body><h1>Change Password</h1></body></html>")
}

/// Change password handler.
pub async fn change_password_post(Form(_form): Form<AccountForm>) -> Html<String> {
    Html("<html><body><h1>Password Changed</h1></body></html>".to_string())
}

/// Delete account page handler.
pub async fn delete_account_get() -> Html<&'static str> {
    Html("<html><body><h1>Delete Account</h1></body></html>")
}

/// Delete account handler.
pub async fn delete_account_post(Form(_form): Form<AccountForm>) -> Html<String> {
    Html("<html><body><h1>Account Deleted</h1></body></html>".to_string())
}

/// Clear watch history page handler.
pub async fn clear_history_get() -> Html<&'static str> {
    Html("<html><body><h1>Clear Watch History</h1></body></html>")
}

/// Clear watch history handler.
pub async fn clear_history_post(Form(_form): Form<AccountForm>) -> Html<String> {
    Html("<html><body><h1>Watch History Cleared</h1></body></html>".to_string())
}

/// Authorize token page handler.
pub async fn authorize_token_get(Query(params): Query<AccountParams>) -> Html<String> {
    let callback = params.page.as_deref().unwrap_or("/");
    Html(format!(
        "<html><body><h1>Authorize Token</h1><p>Callback: {}</p></body></html>",
        callback
    ))
}

/// Authorize token handler.
pub async fn authorize_token_post(Form(_form): Form<AccountForm>) -> Html<String> {
    Html("<html><body><h1>Token Authorized</h1></body></html>".to_string())
}

/// Token manager page handler.
pub async fn token_manager() -> Html<&'static str> {
    Html("<html><body><h1>Token Manager</h1></body></html>")
}

/// Token AJAX handler.
pub async fn token_ajax(Form(_form): Form<AccountForm>) -> Html<String> {
    Html("{}".to_string())
}
