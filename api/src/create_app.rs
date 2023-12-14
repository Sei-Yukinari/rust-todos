use actix_web::{App, guard, HttpResponse, Result, web};
use actix_web::Error;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::Data;
use actix_web::middleware::Logger;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQL, GraphQLRequest, GraphQLResponse};
use sqlx::{Pool, Postgres};
use crate::dependency_injection::dependency_injection;

pub fn create_app(pool: Pool<Postgres>) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response=ServiceResponse<impl MessageBody>,
        Config=(),
        InitError=(),
        Error=Error,
    >,
> {
    let schema = dependency_injection(pool);

    App::new()
        .wrap(Logger::default())
        // .app_data(Data::new(schema.clone()))
        .service(web::resource("/").guard(guard::Post()).to(GraphQL::new(schema.clone())))
        .service(web::resource("/").guard(guard::Get()).to(index_playground))
        .route("/healthz", web::get().to(|| HttpResponse::Ok()))
}

struct Query;

#[Object]
impl Query {
    async fn test(&self) -> usize {
        12345
    }
}

type ApiSchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn index(schema: web::Data<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}