use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use sqlx::{Pool, Postgres};

use crate::{
    gateway::{
        user_repository::PgUserRepository,
    },
    presentation::graphql::{query::Query, schema::build_schema},
    use_case::interactor::{
        query::QueryInteractor,
    },
};

pub type QI = QueryInteractor<PgUserRepository>;

pub fn dependency_injection(
    pool: Pool<Postgres>,
) -> ( Schema<Query<QI>, EmptyMutation, EmptySubscription>) {
    let user_repository = PgUserRepository::new(pool.clone());

    let query_use_case = QueryInteractor {
        user_repository: user_repository.clone(),
    };

    let query = Query::new(query_use_case.clone());

    build_schema(query)
}
