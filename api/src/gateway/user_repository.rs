use async_trait::async_trait;
use sqlx::{PgConnection, PgPool};

use crate::domain::{
    entity::user::User,
    error::DomainError,
    repository::user_repository::UserRepository,
};

#[derive(Debug, Clone, sqlx::FromRow)]
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
        Self {
            pool
        }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        let user = InternalUserRepository::find_by_id(id, &mut conn).await?;
        Ok(user)
    }

    async fn create(&self, name: String) -> Result<Option<User>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        let user = InternalUserRepository::create(name, &mut conn).await?;
        Ok(user)
    }
}

pub(in crate::gateway) struct InternalUserRepository {}

impl InternalUserRepository {
    async fn find_by_id(id: i64, conn: &mut PgConnection) -> Result<Option<User>, DomainError> {
        let row: Option<UserRow> = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(conn)
            .await?;
        Ok(row.map(User::new))
    }

    async fn create(name: String, conn: &mut PgConnection) -> Result<Option<User>, DomainError> {
        println!("name: {}", name);
        let row: Option<UserRow> = sqlx::query_as("INSERT INTO users (name) VALUES ($1) RETURNING *")
            .bind(name)
            .fetch_optional(conn)
            .await?;
        Ok(row.map(User::new))
    }
}


#[cfg(test)]
mod tests {
    use mockall::predicate::*;

    use crate::domain::repository::user_repository::MockUserRepository;

    use super::*;

    #[tokio::test]
    async fn find_by_id_returns_user_when_exists() {
        let mut mock = MockUserRepository::new();
        let expected_user = User::new(
            UserRow {
                id: 1,
                name: "Test User".to_string(),
            }
        );
        let expected_user_clone = expected_user.clone();

        mock.expect_find_by_id()
            .with(eq(1))
            .times(1)
            .return_once(move |_| Ok(Some(expected_user_clone)));

        let user = mock.find_by_id(1).await.unwrap();

        assert_eq!(user, Some(expected_user));
    }

    #[tokio::test]
    async fn find_by_id_returns_none_when_not_exists() {
        let mut mock = MockUserRepository::new();

        mock.expect_find_by_id()
            .with(eq(2))
            .times(1)
            .returning(|_| Ok(None));

        let user = mock.find_by_id(2).await.unwrap();

        assert!(user.is_none());
    }

    #[tokio::test]
    async fn create_returns_user_when_successful() {
        let mut mock = MockUserRepository::new();
        let expected_user = User::new(
            UserRow {
                id: 1,
                name: "Test User".to_string(),
            }
        );
        let expected_user_clone = expected_user.clone();

        mock.expect_create()
            .with(eq("Test User".to_string()))
            .times(1)
            .return_once(move |_| Ok(Some(expected_user_clone)));

        let user = mock.create("Test User".to_string()).await.unwrap();

        assert_eq!(user, Some(expected_user));
    }

    #[tokio::test]
    async fn create_returns_none_when_unsuccessful() {
        let mut mock = MockUserRepository::new();

        mock.expect_create()
            .with(eq("Test User".to_string()))
            .times(1)
            .returning(|_| Ok(None));

        let user = mock.create("Test User".to_string()).await.unwrap();

        assert!(user.is_none());
    }
}