use sqlx::{Pool, Postgres};

use crate::{
    gateway::user_repository::PgUserRepository,
    use_case::interactor::query::QueryInteractor,
};

#[derive(Debug)]
pub struct Context {
    pub query_use_case: QueryInteractor<PgUserRepository>,
}

impl Context {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let user_repository = PgUserRepository::new(pool);
        let query_use_case = QueryInteractor {
            user_repository,
        };
        Self {
            query_use_case,
        }
    }
}