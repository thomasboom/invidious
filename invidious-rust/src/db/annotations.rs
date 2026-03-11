//! Annotation database operations.

use crate::db::DbPool;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Annotation {
    pub id: String,
    pub annotations: Option<String>,
}

pub struct Annotations;

impl Annotations {
    pub async fn insert(pool: &DbPool, annotation: &Annotation) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO annotations (id, annotations) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING")
            .bind(&annotation.id)
            .bind(&annotation.annotations)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &DbPool, id: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM annotations WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn select(pool: &DbPool, id: &str) -> anyhow::Result<Option<Annotation>> {
        let annotation = sqlx::query_as::<_, Annotation>(
            "SELECT id, annotations FROM annotations WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(annotation)
    }

    pub async fn update(pool: &DbPool, id: &str, annotations: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE annotations SET annotations = $1 WHERE id = $2")
            .bind(annotations)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
