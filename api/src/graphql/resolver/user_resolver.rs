use std::sync::Arc;

use async_graphql::{Context, Object, Result};

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