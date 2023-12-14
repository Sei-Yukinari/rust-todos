use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::use_case::traits::query::QueryUseCase;

use super::query::Query;

pub fn build_schema<QUC>(
    query: Query<QUC>,
) -> Schema<Query<QUC>, EmptyMutation, EmptySubscription>
    where
        QUC: QueryUseCase {
    Schema::build(query, EmptyMutation, EmptySubscription).finish()
}