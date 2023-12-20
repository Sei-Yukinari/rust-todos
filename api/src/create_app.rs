use std::sync::Arc;
use actix_web::{App, guard, HttpResponse, Result, web};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::middleware::Logger;
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::GraphQL;
use sqlx::{Pool, Postgres};

use crate::context::Context;
use crate::schema::{create_schema, GraphQLSchema};

pub fn create_app(pool: Pool<Postgres>) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response=ServiceResponse<impl MessageBody>,
        Config=(),
        InitError=(),
        Error=Error,
    >,
> {
    let ctx =  Arc::new(Context::new(pool));
    let schema = create_schema(ctx);

    App::new()
        .wrap(Logger::default())
        .service(web::resource("/").guard(guard::Post()).to(index(schema)))
        .service(web::resource("/").guard(guard::Get()).to(index_playground))
        .route("/healthz", web::get().to(|| HttpResponse::Ok()))
}

fn index(schema: GraphQLSchema) -> GraphQL<GraphQLSchema> {
    GraphQL::new(schema)
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}