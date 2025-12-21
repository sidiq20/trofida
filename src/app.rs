use axum::{Router};
use sqlx::PgPool;

use crate::routes::{auth, todos};

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .merge(auth::routes(pool.clone()))
        .merge(todos::routes(pool))
}