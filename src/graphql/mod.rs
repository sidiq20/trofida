use async_graphql::{Object};

pub struct QueryRoot;

#[Object]
impl QueryRoot (
    async fn health(&self) -> &str {
        "OK"
    }
)

pub struct MutationRoot;

