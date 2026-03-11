//! Database migration runner.

use crate::db::DbPool;
use sqlx::Row;

const MIGRATIONS_TABLE: &str = "invidious_migrations";

pub struct Migration {
    pub version: i64,
    pub name: String,
}

pub struct Migrations;

impl Migrations {
    pub async fn migrate(pool: &DbPool) -> anyhow::Result<()> {
        Self::create_migrations_table(pool).await?;

        let versions = Self::load_versions(pool).await?;
        let migrations = Self::get_migrations();

        for migration in migrations {
            if !versions.contains(&migration.version) {
                tracing::info!("Running migration: {}", migration.name);
                Self::run_migration(pool, migration.version).await?;
            }
        }

        tracing::info!("Migrations complete");
        Ok(())
    }

    pub async fn pending_migrations(pool: &DbPool) -> anyhow::Result<Vec<Migration>> {
        Self::create_migrations_table(pool).await?;

        let versions = Self::load_versions(pool).await?;
        let migrations = Self::get_migrations();

        let pending: Vec<Migration> = migrations
            .into_iter()
            .filter(|m| !versions.contains(&m.version))
            .collect();

        Ok(pending)
    }

    async fn create_migrations_table(pool: &DbPool) -> anyhow::Result<()> {
        sqlx::query(&format!(
            "CREATE TABLE IF NOT EXISTS {} (id bigserial PRIMARY KEY, version bigint NOT NULL)",
            MIGRATIONS_TABLE
        ))
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn load_versions(pool: &DbPool) -> anyhow::Result<Vec<i64>> {
        let rows = sqlx::query(&format!("SELECT version FROM {}", MIGRATIONS_TABLE))
            .fetch_all(pool)
            .await?;

        let versions: Vec<i64> = rows.iter().map(|row| row.get::<i64, _>(0)).collect();
        Ok(versions)
    }

    async fn run_migration(pool: &DbPool, version: i64) -> anyhow::Result<()> {
        let migration_sql = match version {
            1 => include_str!("../../sql/migrations/0001_channels.sql"),
            2 => include_str!("../../sql/migrations/0002_videos.sql"),
            3 => include_str!("../../sql/migrations/0003_channel_videos.sql"),
            4 => include_str!("../../sql/migrations/0004_users.sql"),
            5 => include_str!("../../sql/migrations/0005_session_ids.sql"),
            6 => include_str!("../../sql/migrations/0006_nonces.sql"),
            7 => include_str!("../../sql/migrations/0007_annotations.sql"),
            8 => include_str!("../../sql/migrations/0008_playlists.sql"),
            9 => include_str!("../../sql/migrations/0009_playlist_videos.sql"),
            10 => include_str!("../../sql/migrations/0010_videos_unlogged.sql"),
            _ => return Err(anyhow::anyhow!("Unknown migration version: {}", version)),
        };

        for statement in migration_sql.split(';').filter(|s| !s.trim().is_empty()) {
            sqlx::query(statement).execute(pool).await?;
        }

        sqlx::query(&format!("INSERT INTO {} (version) VALUES ($1)", MIGRATIONS_TABLE))
            .bind(version)
            .execute(pool)
            .await?;

        Ok(())
    }

    fn get_migrations() -> Vec<Migration> {
        vec![
            Migration { version: 1, name: "create_channels_table".to_string() },
            Migration { version: 2, name: "create_videos_table".to_string() },
            Migration { version: 3, name: "create_channel_videos_table".to_string() },
            Migration { version: 4, name: "create_users_table".to_string() },
            Migration { version: 5, name: "create_session_ids_table".to_string() },
            Migration { version: 6, name: "create_nonces_table".to_string() },
            Migration { version: 7, name: "create_annotations_table".to_string() },
            Migration { version: 8, name: "create_playlists_table".to_string() },
            Migration { version: 9, name: "create_playlist_videos_table".to_string() },
            Migration { version: 10, name: "make_videos_unlogged".to_string() },
        ]
    }
}
