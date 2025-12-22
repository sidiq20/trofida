use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn connect_db(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5)) // ✅ supported
        .connect(database_url)
        .await
}
