use axum::{Router, routing::get};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::GraphiQLSource;
use axum::response::Html;
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
    axum::extract::Extension(schema): axum::extract::Extension<schema::AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> Html<String> {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");

    // ✅ unwrap ONCE here
    let pool = db::connect_db(&db_url)
        .await
        .expect("Failed to connect to database");

    let schema = schema::build_schema(pool);

    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .layer(axum::Extension(schema));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 GraphQL running at http://localhost:3000/graphql");

    axum::serve(listener, app).await.unwrap();
}
