//! Login and authentication routes for Invidious.
//!
//! Handles user login, signout, and session management.

use axum::{
    extract::{Form, Query},
    response::Html,
};
use serde::Deserialize;

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
pub async fn login_page(Query(params): Query<LoginParams>) -> Html<String> {
    let referer = params.r.as_deref().unwrap_or("/");
    Html(format!(
        "<html><body><h1>Login</h1><form method='post'><input name='email'/><input name='password' type='password'/><input type='submit'/></form><p>Referer: {}</p></body></html>",
        referer
    ))
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
