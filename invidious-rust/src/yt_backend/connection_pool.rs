//! Connection pool module for YouTube API requests.
//!
//! Provides connection pooling for efficient HTTP communication with YouTube.

use reqwest::Client;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

/// A connection pool for YouTube API requests.
pub struct ConnectionPool {
    client: Client,
    subdomain_pools: RwLock<HashMap<String, Arc<SubdomainPool>>>,
    capacity: usize,
    timeout_secs: f64,
}

impl ConnectionPool {
    /// Create a new connection pool with the specified capacity.
    pub fn new(capacity: usize, timeout_secs: f64) -> anyhow::Result<Self> {
        let client = Client::builder()
            .pool_max_idle_per_host(capacity)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .tcp_nodelay(true)
            .build()?;

        Ok(Self {
            client,
            subdomain_pools: RwLock::new(HashMap::new()),
            capacity,
            timeout_secs,
        })
    }

    /// Get the HTTP client for making requests.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a subdomain-specific client pool.
    pub async fn get_subdomain_pool(&self, subdomain: &str) -> Arc<SubdomainPool> {
        let mut pools = self.subdomain_pools.write().await;
        
        if let Some(pool) = pools.get(subdomain) {
            return Arc::clone(pool);
        }

        let pool = Arc::new(SubdomainPool::new(
            format!("https://{}.ytimg.com", subdomain),
            self.capacity,
            self.timeout_secs,
        ));

        pools.insert(subdomain.to_string(), Arc::clone(&pool));
        Arc::clone(&pool)
    }
}

/// A subdomain-specific connection pool for ytimg.com requests.
pub struct SubdomainPool {
    url: String,
    client: Client,
}

impl SubdomainPool {
    /// Create a new subdomain pool.
    pub fn new(url: String, capacity: usize, _timeout_secs: f64) -> Self {
        let client = Client::builder()
            .pool_max_idle_per_host(capacity)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .tcp_nodelay(true)
            .build()
            .expect("Failed to build subdomain client");

        Self { url, client }
    }

    /// Get the client for this subdomain.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get the URL for this subdomain.
    pub fn url(&self) -> &str {
        &self.url
    }
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self::new(5, 5.0).expect("Failed to create default ConnectionPool")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_pool_creation() {
        let pool = ConnectionPool::new(10, 10.0);
        assert!(pool.is_ok());
    }

    #[test]
    fn test_subdomain_pool_creation() {
        let pool = SubdomainPool::new("https://i.ytimg.com".to_string(), 5, 5.0);
        assert_eq!(pool.url(), "https://i.ytimg.com");
    }
}
