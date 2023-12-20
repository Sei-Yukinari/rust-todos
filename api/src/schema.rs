use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};
use crate::context::Context;
use crate::graphql::resolver::user_resolver::UsersQuery;

#[derive(MergedObject, Default)]
pub struct Query(UsersQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(EmptyMutation);

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(
    ctx: Context,
) -> GraphQLSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(ctx)
        .finish()
}
