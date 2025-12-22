use async_graphql::{Object, Context};
use sqlx::PgPool;
use crate::models::todo::{Todo, TodoStatus};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health(&self) -> &str {
        "OK"
    }

    async fn todos(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Todo>> {
        let pool = ctx.data::<PgPool>()?;
        
        let todos = sqlx::query_as!(
            Todo,
            r#"
            SELECT 
                id, 
                user_id as "user_id!", 
                title, 
                streak, 
                last_completed, 
                streak_required, 
                created_at as "created_at!", 
                status as "status!: TodoStatus"
            FROM todos
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(todos)
    }

    async fn my_todos(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Todo>> {
        let pool = ctx.data::<PgPool>()?;
        let user = ctx.data::<crate::utils::authuser::AuthUser>()
            .map_err(|_| async_graphql::Error::new("Unauthorized"))?;

        let todos = sqlx::query_as!(
            Todo,
            r#"
            SELECT 
                id, 
                user_id as "user_id!", 
                title, 
                streak, 
                last_completed, 
                streak_required, 
                created_at as "created_at!", 
                status as "status!: TodoStatus"
            FROM todos
            WHERE user_id = $1
            "#,
            user.id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(todos)
    }
}
