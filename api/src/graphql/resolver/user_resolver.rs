use async_graphql::{Context, Object, Result};

use crate::{
    presentation::graphql::object::User,
    use_case::traits::query::QueryUseCase,
};

#[derive(Default)]
pub struct UsersQuery {}

#[Object]
impl UsersQuery {
    async fn logged_in_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let usecase = &ctx
            .data_unchecked::<crate::context::Context>()
            .query_use_case;
        let user = usecase
            .find_user_by_id(1)
            .await?
            .map(User::new);
        Ok(user)
    }
}