use async_trait::async_trait;

use crate::{
    domain::repository::user_repository::UserRepository,
    use_case::traits::user::RegisterUserUseCase,
};

#[warn(dead_code)]
pub struct RegisterUserInteractor<UR> {
    user_repository: UR,
}

impl<UR> RegisterUserInteractor<UR> {
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<UR> RegisterUserUseCase for RegisterUserInteractor<UR>
    where
        UR: UserRepository,
{}
