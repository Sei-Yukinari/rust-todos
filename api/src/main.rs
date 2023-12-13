use std::env;
use std::net::SocketAddr;

use actix_cors::Cors;
use actix_web::{App, guard, http, HttpResponse, HttpServer, Result, web};
use actix_web::web::Data;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::postgres::PgPoolOptions;

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

    let addr = SocketAddr::from(([0, 0, 0, 0], fetch_env_var("PORT", Some(8080))));
    println!("Playground: http://{}", addr);
    let db_url = fetch_env_var(
        "DATABASE_URL",
        Some("postgres://root:postgres@localhost:5432/dev".to_string()),
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migration failed.");

    println!("Database: {}", db_url);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
            .route("/healthz", web::get().to(|| HttpResponse::Ok()))
    })
        .bind(addr)?
        .run()
        .await
}

fn fetch_env_var<T: std::str::FromStr>(var_name: &str, default: Option<T>) -> T
    where
        T: std::fmt::Debug,
{
    match env::var(var_name) {
        Ok(s) => s
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse {}: {:?}", var_name, s)),
        Err(env::VarError::NotPresent) => {
            if let Some(default_value) = default {
                default_value
            } else {
                panic!("Environment variable {} is required.", var_name);
            }
        }
        Err(env::VarError::NotUnicode(_)) => {
            panic!("Environment variable {} is not unicode.", var_name)
        }
    }
}
