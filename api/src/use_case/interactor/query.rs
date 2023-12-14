use async_trait::async_trait;

use crate::{
    domain::{
        entity::user::UserId,
        repository::user_repository::UserRepository,
    },
    use_case::{
        dto::user::UserDto,
        error::UseCaseError,
        traits::query::QueryUseCase,
    },
};

#[derive(Debug, Clone)]
pub struct QueryInteractor<UR> {
    pub user_repository: UR,
}

#[async_trait]
impl<UR> QueryUseCase for QueryInteractor<UR>
    where
        UR: UserRepository,
{
    async fn find_user_by_id(&self, raw_user_id: i64) -> Result<Option<UserDto>, UseCaseError> {
        let user = self.user_repository.find_by_id(raw_user_id).await?;
        Ok(user.map(|user| UserDto::new(user)))
    }
}