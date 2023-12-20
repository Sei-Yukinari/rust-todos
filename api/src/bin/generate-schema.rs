use std::io::Write;
use api::context::Context;
// use api::{
//     context::Context,
// };
use api::infrastructure::db::postgres::db_pool;
use api::schema::create_schema;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = db_pool().await;
    let ctx = Context::new(pool);
    let schema = create_schema(ctx);
    let mut sdl_file = std::fs::File::create("./schema/schema.graphql")?;
    write!(&mut sdl_file, "{}", &schema.sdl()).expect("error");
    println!("{}", schema.sdl());
    Ok(())
}
