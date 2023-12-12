use std::env;
use std::net::SocketAddr;

use actix_web::{App, guard, HttpResponse, HttpServer, Result, web};
use actix_web::web::Data;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let addr = SocketAddr::from(([0, 0, 0, 0],
                                 fetch_env_var("PORT", Some(8080))));
    println!("Playground: http://{}", addr);
    let db_url = fetch_env_var("DATABASE_URL", Some("postgres://root:postgres@localhost:5432/dev".to_string()));
    println!("Database: {}", db_url);
    let allowed_origins: Vec<String> = fetch_env_var("ALLOWED_ORIGINS", Some("*".to_string()))
        .split(',')
        .map(|s| s.to_string())
        .collect();
    println!("Allowed origins: {:?}", allowed_origins);


    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
            .route("/healthz", web::get().to(|| HttpResponse::Ok()))
    })
        .bind(addr)?
        .run()
        .await
}

fn fetch_env_var<T: std::str::FromStr>(
    var_name: &str,
    default: Option<T>,
) -> T
    where
        T: std::fmt::Debug,
{
    match env::var(var_name) {
        Ok(s) => s.parse().unwrap_or_else(|_| panic!("Failed to parse {}: {:?}", var_name, s)),
        Err(env::VarError::NotPresent) => {
            if let Some(default_value) = default {
                default_value
            } else {
                panic!("Environment variable {} is required.", var_name);
            }
        }
        Err(env::VarError::NotUnicode(_)) => panic!("Environment variable {} is not unicode.", var_name),
    }
}