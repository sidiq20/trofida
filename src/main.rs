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
mod routes;
mod handlers;

use axum::http::HeaderMap;

async fn graphql_handler(
    axum::extract::Extension(schema): axum::extract::Extension<schema::AppSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    
    // Attempt to extract potential AuthUser from headers
    if let Some(auth_header) = headers.get(axum::http::header::AUTHORIZATION) {
        if let Ok(token_str) = auth_header.to_str() {
             if let Some(token) = token_str.strip_prefix("Bearer ") {
                 if let Ok(claims) = utils::jwt::decode_jwt(token) {
                     let user = utils::authuser::AuthUser { id: claims.sub };
                     request = request.data(user);
                 }
             }
        }
    }

    schema.execute(request).await.into()
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
