use std::sync::Arc;

use async_graphql::{EmptySubscription, MergedObject, Schema};

use crate::context::Context;
use crate::graphql::resolver::user_resolver::{UsersMutation, UsersQuery};

#[derive(MergedObject, Default)]
pub struct Query(UsersQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UsersMutation);

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(
    ctx: Arc<Context>,
) -> GraphQLSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(ctx)
        .finish()
}
