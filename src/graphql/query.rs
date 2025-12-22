use async_graphql::{Object, Context};
use sqlx::PgPool;
use crate::models::todo::Todo;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health(&self) -> &str {
        "OK"
    }

    async fn todos(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Todo>> {
        let pool = ctx.data::<PgPool>()?;
        // Removed user_id check for simplicity as per previous implementation, 
        // OR we can add Authorizaion check here if we want to be strict.
        // For now matching previous simple list behavior but enabling DB access.
        
        let todos = sqlx::query_as::<_, Todo>(
            "SELECT id, user_id, title, streak, last_completed, streak_required, created_at FROM todos"
        )
        .fetch_all(pool)
        .await?;
        
        Ok(todos)
    }
}
