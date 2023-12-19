use std::io::Write;

use api::{
    presentation::graphql::{query::Query, schema::build_schema},
    use_case::traits::query::MockQueryUseCase,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let query_use_case = MockQueryUseCase::new();
    let query = Query::new(query_use_case);
    let schema = build_schema(query);

    let mut sdl_file = std::fs::File::create("./schema/schema.graphql")?;
    write!(&mut sdl_file, "{}", &schema.sdl()).expect("error");
    println!("{}", schema.sdl());
    Ok(())
}
