use async_trait::async_trait;
use mockall::automock;

use crate::use_case::{
    dto::user::UserDto,
    error::UseCaseError,
};

#[automock]
#[async_trait]
pub trait QueryUseCase: Send + Sync {
    async fn find_user_by_id(&self, user_id: i64) -> Result<Option<UserDto>, UseCaseError>;
}
