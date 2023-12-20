use async_trait::async_trait;

use crate::{
    domain::repository::user_repository::UserRepository,
    use_case::traits::user::UserUseCase,
};
use crate::use_case::dto::user::UserDto;
use crate::use_case::error::UseCaseError;

#[derive(Debug)]
pub struct Interactor<U> {
    pub user_repository: U,
}

impl<U> Interactor<U> {
    pub fn new(user_repository: U) -> Self {
        Self {
            user_repository
        }
    }
}

#[async_trait]
impl<U> UserUseCase for Interactor<U>
    where
        U: UserRepository,
{
    async fn find_user_by_id(&self, raw_user_id: i64) -> Result<Option<UserDto>, UseCaseError> {
        let user = self.user_repository.find_by_id(raw_user_id).await?;
        Ok(user.map(|user| UserDto::new(user)))
    }
}
