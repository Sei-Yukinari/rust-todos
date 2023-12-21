use async_trait::async_trait;

use crate::use_case::dto::user::UserDto;
use crate::use_case::error::UseCaseError;

#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    async fn find_user_by_id(&self, user_id: i64) -> Result<Option<UserDto>, UseCaseError>;
    async fn create(&self, name: String) -> Result<Option<UserDto>, UseCaseError>;
}
