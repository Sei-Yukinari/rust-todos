use async_graphql::{Context, ID, Object};

use crate::{
    presentation::error::PresentationalError,
    use_case::traits::query::QueryUseCase,
};

use super::object::User;

pub struct Query<QUC> {
    query_use_case: QUC,
}

impl<QUC> Query<QUC> {
    pub fn new(query_use_case: QUC) -> Self {
        Query { query_use_case }
    }
}

#[Object]
impl<QUC> Query<QUC>
    where
        QUC: QueryUseCase,
{
    async fn logged_in_user(&self) -> Result<Option<User>, PresentationalError> {
        let user = self.query_use_case.find_user_by_id(1).await?;
        Ok(user.map(|user| User::new(user)))
    }

}