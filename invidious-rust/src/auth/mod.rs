//! Authentication service for Invidious.
//!
//! Provides password hashing, verification, and session management.

use crate::db::DbPool;
use crate::db::sessions::SessionIds;
use crate::db::users::Users;
use crate::config::Config;
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Authenticated user data extracted from session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub username: String,
}

/// Authentication errors.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Login disabled")]
    LoginDisabled,
    #[error("Registration disabled")]
    RegistrationDisabled,
    #[error("Session expired or invalid")]
    InvalidSession,
    #[error("Database error: {0}")]
    Database(#[from] anyhow::Error),
}

/// Result type for authentication operations.
pub type AuthResult<T> = Result<T, AuthError>;

/// Authentication service for handling user authentication.
pub struct AuthService {
    db: DbPool,
    config: Config,
}

impl AuthService {
    /// Create a new authentication service.
    pub fn new(db: DbPool, config: Config) -> Self {
        Self { db, config }
    }

    /// Verify a user's password and create a session.
    pub async fn login(&self, email: &str, password: &str) -> AuthResult<(String, String)> {
        if !self.config.login_enabled {
            return Err(AuthError::LoginDisabled);
        }

        let email_lower = email.to_lowercase();
        let email_bytes = email_lower.as_bytes();
        let email_truncated = if email_bytes.len() > 254 {
            &email_bytes[..254]
        } else {
            email_bytes
        };
        let email_str = String::from_utf8_lossy(email_truncated).to_string();

        let user = Users::select_by_email(&self.db, &email_str)
            .await
            .map_err(AuthError::Database)?;

        match user {
            Some(user) => {
                if let Some(stored_hash) = &user.password {
                    if self.verify_password(password, stored_hash)? {
                        let sid = Self::generate_sid();
                        SessionIds::insert(&self.db, &sid, &email_str, false).await?;

                        return Ok((sid, email_str));
                    }
                }
                Err(AuthError::InvalidPassword)
            }
            None => {
                if !self.config.registration_enabled {
                    return Err(AuthError::RegistrationDisabled);
                }
                Err(AuthError::UserNotFound)
            }
        }
    }

    /// Register a new user and create a session.
    pub async fn register(&self, email: &str, password: &str) -> AuthResult<(String, String)> {
        if !self.config.login_enabled {
            return Err(AuthError::LoginDisabled);
        }

        if !self.config.registration_enabled {
            return Err(AuthError::RegistrationDisabled);
        }

        let email_lower = email.to_lowercase();
        let email_bytes = email_lower.as_bytes();
        let email_truncated = if email_bytes.len() > 254 {
            &email_bytes[..254]
        } else {
            email_bytes
        };
        let email_str = String::from_utf8_lossy(email_truncated).to_string();

        if Users::select_by_email(&self.db, &email_str)
            .await
            .map_err(AuthError::Database)?
            .is_some()
        {
            return Err(AuthError::UserNotFound);
        }

        let password_bytes = password.as_bytes();
        let password_truncated = if password_bytes.len() > 55 {
            &password_bytes[..55]
        } else {
            password_bytes
        };
        let password_str = String::from_utf8_lossy(password_truncated).to_string();

        let password_hash = self.hash_password(&password_str)?;

        let user = crate::db::users::User {
            updated: Some(Utc::now()),
            notifications: Some(vec![]),
            subscriptions: Some(vec![]),
            email: email_str.clone(),
            preferences: Some(serde_json::to_string(&self.config.default_user_preferences).unwrap_or_default()),
            password: Some(password_hash),
            token: None,
            watched: Some(vec![]),
            feed_needs_update: Some(false),
        };

        Users::insert(&self.db, &user, false).await?;

        let sid = Self::generate_sid();
        SessionIds::insert(&self.db, &sid, &email_str, false).await?;

        Ok((sid, email_str))
    }

    /// Logout a user by deleting their session.
    pub async fn logout(&self, sid: &str) -> AuthResult<()> {
        SessionIds::delete_by_sid(&self.db, sid).await?;
        Ok(())
    }

    /// Get user from session ID.
    pub async fn get_user_from_session(&self, sid: &str) -> AuthResult<Option<AuthUser>> {
        let email = SessionIds::select_email(&self.db, sid)
            .await
            .map_err(AuthError::Database)?;

        match email {
            Some(email) => {
                let user = Users::select_by_email(&self.db, &email)
                    .await
                    .map_err(AuthError::Database)?;

                Ok(user.map(|u| AuthUser {
                    username: u.email.clone(),
                    email: u.email,
                }))
            }
            None => Ok(None),
        }
    }

    /// Update user password.
    pub async fn update_password(&self, email: &str, old_password: &str, new_password: &str) -> AuthResult<()> {
        let user = Users::select_by_email(&self.db, email)
            .await
            .map_err(AuthError::Database)?
            .ok_or(AuthError::UserNotFound)?;

        if let Some(stored_hash) = &user.password {
            if !self.verify_password(old_password, stored_hash)? {
                return Err(AuthError::InvalidPassword);
            }
        } else {
            return Err(AuthError::InvalidPassword);
        }

        let password_bytes = new_password.as_bytes();
        let password_truncated = if password_bytes.len() > 55 {
            &password_bytes[..55]
        } else {
            password_bytes
        };
        let password_str = String::from_utf8_lossy(password_truncated).to_string();
        let password_hash = self.hash_password(&password_str)?;

        Users::update_password(&self.db, email, &password_hash).await?;

        Ok(())
    }

    /// Delete a user account.
    pub async fn delete_account(&self, email: &str) -> AuthResult<()> {
        Users::delete(&self.db, email).await?;
        SessionIds::delete_by_email(&self.db, email).await?;
        Ok(())
    }

    /// Hash a password using bcrypt.
    fn hash_password(&self, password: &str) -> AuthResult<String> {
        let hash = bcrypt::hash(password, 10)
            .map_err(|e| anyhow::anyhow!("bcrypt error: {}", e))?;
        Ok(hash)
    }

    /// Verify a password against a bcrypt hash.
    fn verify_password(&self, password: &str, hash: &str) -> AuthResult<bool> {
        let password_bytes = password.as_bytes();
        let password_truncated = if password_bytes.len() > 55 {
            &password_bytes[..55]
        } else {
            password_bytes
        };
        let password_str = String::from_utf8_lossy(password_truncated).to_string();

        Ok(bcrypt::verify(password_str, hash).unwrap_or(false))
    }

    /// Generate a random session ID.
    fn generate_sid() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let mut buf = [0u8; 32];
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        for (i, byte) in buf.iter_mut().enumerate() {
            let val = ((timestamp >> (i % 8)) ^ (i as u128 * 0x9e3779b97f4a7c15)) as u8;
            *byte = val.wrapping_mul(0x4b).wrapping_add(val);
        }
        
        base64_encode(&buf)
    }
}

/// Encode bytes to URL-safe base64.
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    
    let mut result = String::new();
    let mut i = 0;
    
    while i < data.len() {
        let b0 = data[i] as usize;
        let b1 = if i + 1 < data.len() { data[i + 1] as usize } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] as usize } else { 0 };
        
        result.push(ALPHABET[b0 >> 2] as char);
        result.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);
        
        if i + 1 < data.len() {
            result.push(ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }
        
        if i + 2 < data.len() {
            result.push(ALPHABET[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }
        
        i += 3;
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let test_data = b"Hello";
        let encoded = base64_encode(test_data);
        assert_eq!(encoded, "SGVsbG8");
    }
}
