use std::sync::Arc;

use async_graphql::{Context, InputObject, Object, Result};

use crate::context::Context as AppContext;
use crate::presentation::graphql::user::User;
use crate::use_case::traits::user::UserUseCase;

#[derive(Default)]
pub struct UsersQuery {}

#[Object]
impl UsersQuery {
    async fn logged_in_user(&self, ctx: &Context<'_>, id: i64) -> Result<Option<User>> {
        let usecase = &ctx
            .data_unchecked::<Arc<AppContext>>()
            .user_use_case;
        let user = usecase
            .find_user_by_id(id)
            .await?
            .map(User::new);
        Ok(user)
    }
}

#[derive(Default)]
pub struct UsersMutation {}

#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct CreateUserInput {
    /// The User's profile
    pub name: String,
}

#[Object]
impl UsersMutation {
    async fn register_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<Option<User>> {
        let usecase = &ctx
            .data_unchecked::<Arc<AppContext>>()
            .user_use_case;
        let user = usecase
            .create(input.name)
            .await?
            .map(User::new);
        Ok(user)
    }
}
