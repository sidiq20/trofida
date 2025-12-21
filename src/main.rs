use axum::{Router, routing::get};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;

mod schema;
mod graphql;
mod db;
mod models;
mod utils;
mod errors;

async fn graphql_handler(
    schema: axum::extract::Extension<schema::AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");
    let pool = db::connect_db(&db_url).await;

    let schema = schema::build_schema(pool);

    let app = Router::new()
        .route("/graphql", get(async_graphql_axum::graphiql).post(graphql_handler))
        .layer(axum::Extension(schema));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 GraphQL running at http://localhost:3000/graphql");

    axum::serve(listener, app).await.unwrap();
}
