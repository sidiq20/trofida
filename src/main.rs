mod db;
mod errors;
mod graphql;
mod models;
mod domains; 
mod schema;
mod utils;

use axum::{
    routing::{get, post},
    Router, Extension,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use axum::response::{Html, IntoResponse};
use std::env;
use tokio::net::TcpListener;
use db::connect_db;
use schema::{build_schema, AppSchema};

use tracing::{info, warn, error};

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    headers: axum::http::HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                 match crate::utils::jwt::decode_jwt(token) {
                    Ok(claims) => {
                        req = req.data(crate::utils::authuser::AuthUser { id: claims.sub });
                    }
                    Err(e) => {
                        warn!("Failed to decode JWT: {}", e);
                    }
                }
            } else {
                warn!("Authorization header missing 'Bearer ' prefix. Received: {}", auth_str);
            }
        }
    }

    schema.execute(req).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = connect_db(&database_url).await.expect("Failed to connect to DB");

    let schema = build_schema(pool);

    let app = Router::new()
        .route("/", get(graphql_playground))
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema));

    println!("Server started at http://localhost:3001");

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
