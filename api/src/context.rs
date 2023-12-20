use sqlx::{Pool, Postgres};

use crate::{
    gateway::user_repository::PgUserRepository,
    use_case::interactor::user::Interactor,
};

#[derive(Debug)]
pub struct Context {
    pub user_use_case: Interactor<PgUserRepository>,
}

impl Context {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let user_repository = PgUserRepository::new(pool);
        let user_use_case = Interactor {
            user_repository,
        };
        Self {
            user_use_case,
        }
    }
}