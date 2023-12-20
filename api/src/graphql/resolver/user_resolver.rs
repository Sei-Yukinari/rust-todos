use async_graphql::{Context, Object, Result};

use crate::{
    presentation::graphql::user::User,
};
use crate::use_case::traits::user::UserUseCase;


#[derive(Default)]
pub struct UsersQuery {}

#[Object]
impl UsersQuery {
    async fn logged_in_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let usecase = &ctx
            .data_unchecked::<crate::context::Context>()
            .user_use_case;
        let user = usecase
            .find_user_by_id(1)
            .await?
            .map(User::new);
        Ok(user)
    }
}