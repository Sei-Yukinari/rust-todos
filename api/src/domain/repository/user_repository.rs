use async_trait::async_trait;
use mockall::automock;

use crate::domain::{
    entity::user::{User, UserId},
    error::DomainError,
};

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, DomainError>;
}
