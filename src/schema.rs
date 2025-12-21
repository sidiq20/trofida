use async_graphql::{Schema};
use sqlx::PgPool;

use crate::graphql::{QueryRoot, MutationRoot};

pub type AppSchema = Schema<QueryRoot, MutationRoot, ()>;

pub fn build_schema(pool: PgPool) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, ())
    .data(pool)
    .finish()
}