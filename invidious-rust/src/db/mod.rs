//! Database module for Invidious.
//!
//! Provides database access using PostgreSQL.

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

/// Database connection pool.
pub type DbPool = Pool<Postgres>;

/// Initialize the database connection pool.
pub async fn init_db(database_url: &str) -> anyhow::Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}
