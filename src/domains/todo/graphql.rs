use async_graphql::{Object, Context, Error};
use sqlx::PgPool;
use uuid::Uuid;
use super::models::{Todo, CreateTodo, TodoStatus};

#[derive(Default)]
pub struct TodoQuery;

#[Object]
impl TodoQuery {
    async fn todos(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Todo>> {
        let pool = ctx.data::<PgPool>()?;
        
        let todos = sqlx::query_as!(
            Todo,
            r#"
            SELECT 
                id, 
                user_id as "user_id!", 
                folder_id,
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
                folder_id,
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

#[derive(Default)]
pub struct TodoMutation;

#[Object]
impl TodoMutation {
    async fn create_todo(&self, ctx: &Context<'_>, input: CreateTodo) -> async_graphql::Result<Todo> {
        let pool = ctx.data::<PgPool>()?;
        let user = ctx.data::<crate::utils::authuser::AuthUser>()
            .map_err(|_| Error::new("Unauthorized: Please log in"))?;
        
        let todo = sqlx::query_as!(
            Todo,
            r#"
            INSERT INTO todos (id, user_id, title, streak_required, streak)
            VALUES ($1, $2, $3, $4, 0)
            RETURNING id, user_id as "user_id!", folder_id, title, streak, last_completed, streak_required, created_at as "created_at!", status as "status!: TodoStatus"
            "#,
            Uuid::new_v4(),
            user.id,
            input.title,
            input.streak_required
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::new(e.to_string()))?;

        Ok(todo)
    }

    async fn mark_completed(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Todo> {
        let pool = ctx.data::<PgPool>()?;
        let user = ctx.data::<crate::utils::authuser::AuthUser>()
            .map_err(|_| Error::new("Unauthorized"))?;
        
        let todo = sqlx::query_as!(
            Todo,
            r#"UPDATE todos 
            SET last_completed = NOW(), streak = streak + 1 
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id as "user_id!", folder_id, title, streak, last_completed, streak_required, created_at as "created_at!", status as "status!: TodoStatus""#,
            id,
            user.id
        )
        .fetch_optional(pool) 
        .await
        .map_err(|e| Error::new(e.to_string()))?
        .ok_or_else(|| Error::new("Todo not found or authorized"))?;

        Ok(todo)
    }

    async fn delete_todo(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        let user = ctx.data::<crate::utils::authuser::AuthUser>()
            .map_err(|_| Error::new("Unauthorized"))?;

        let result = sqlx::query!(
            "DELETE FROM todos WHERE id = $1 AND user_id = $2",
            id,
            user.id
        )
        .execute(pool)
        .await
        .map_err(|e| Error::new(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(Error::new("Todo not found or authorized"));
        }

        Ok(true)
    }
}
