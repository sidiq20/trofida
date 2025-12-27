use async_graphql::{Schema, EmptySubscription};
use sqlx::PgPool;
use crate::graphql::{QueryRoot, MutationRoot};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(pool: PgPool) -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(pool)
        .finish()
}
