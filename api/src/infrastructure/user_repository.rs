use async_trait::async_trait;
use sqlx::{PgConnection, PgPool};

use crate::domain::{
    entity::user::{User, UserId},
    error::DomainError,
    repository::user_repository::UserRepository,
};

#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub(crate) id: i64,
    pub(crate) name: String,
}

#[derive(Debug, Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        let user = InternalUserRepository::find_by_id(id, &mut conn).await?;
        Ok(user)
    }
}

pub(in crate::infrastructure) struct InternalUserRepository {}

impl InternalUserRepository {
    async fn find_by_id(id: i64, conn: &mut PgConnection) -> Result<Option<User>, DomainError> {
        let row: Option<UserRow> = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(1)
            .fetch_optional(conn)
            .await?;
        Ok(row.map(|row| User::new(row)))
    }
}

