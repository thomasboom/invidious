//! Authentication middleware for Invidious.
//!
//! Provides middleware to extract user from session and add to request extensions.

use crate::auth::{AuthService, AuthUser};
use crate::routes::api::AppState;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

/// Key for storing authenticated user in request extensions.
pub const AUTH_USER_KEY: &str = "auth_user";

/// Key for storing session ID in request extensions.
pub const SID_KEY: &str = "sid";

/// Middleware to extract user from session cookie and add to request extensions.
pub async fn auth_middleware(
    state: axum::Extension<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let sid = get_sid_from_cookies(&request);

    if let Some(sid) = sid {
        if let Some(ref db) = state.db {
            let auth_service = AuthService::new(db.clone(), state.config.clone());
            
            match auth_service.get_user_from_session(&sid).await {
                Ok(Some(user)) => {
                    request.extensions_mut().insert(user.clone());
                    request.extensions_mut().insert(sid.clone());
                }
                Ok(None) => {}
                Err(_) => {}
            }
        }
    }

    next.run(request).await
}

/// Middleware to require authentication for a route.
pub async fn require_auth_middleware(
    _state: axum::Extension<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let user = request.extensions().get::<AuthUser>();

    if user.is_none() {
        return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
    }

    next.run(request).await
}

/// Extract session ID from cookies.
pub fn get_sid_from_cookies(request: &Request) -> Option<String> {
    let cookies = request.headers().get("cookie")?;
    let cookie_str = cookies.to_str().ok()?;
    
    for cookie in cookie_str.split(';') {
        let cookie = cookie.trim();
        if cookie.starts_with("SID=") {
            return Some(cookie[4..].to_string());
        }
    }
    
    None
}

/// Helper to get authenticated user from request extensions.
pub fn get_auth_user<B>(request: &Request<B>) -> Option<AuthUser> {
    request.extensions().get::<AuthUser>().cloned()
}

/// Helper to get session ID from request extensions.
pub fn get_sid<B>(request: &Request<B>) -> Option<String> {
    request.extensions().get::<String>().cloned()
}
