use async_graphql::{Object, Context};
use sqlx::PgPool;
use crate::models::todo::{Todo, CreateTodo};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health(&self) -> &str {
        "OK"
    }

    async fn todos(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Todo>> {
        let pool = ctx.data::<PgPool>()?;
        let todos = sqlx::query_as::<_, Todo>("SELECT id, title, streak, last_completed FROM todos")
            .fetch_all(pool)
            .await?;
        Ok(todos)
    }
}



pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_todo(&self, ctx: &Context<'_>, input: CreateTodo) -> async_graphql::Result<Todo> {
        let pool = ctx.data::<PgPool>()?;
        let todo = sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (id, title, streak) VALUES ($1, $2, 0) RETURNING id, title, streak, last_completed"
        )
        .bind(uuid::Uuid::new_v4())
        .bind(input.title)
        .fetch_one(pool)
        .await?;
        Ok(todo)
    }

    async fn mark_completed(&self, ctx: &Context<'_>, id: uuid::Uuid) -> async_graphql::Result<Todo> {
        let pool = ctx.data::<PgPool>()?;
        let updated_todo = sqlx::query_as::<_, Todo>(
            "UPDATE todos SET completed = true WHERE id = $1 RETURNING id, title, streak, last_completed"
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        Ok(updated_todo)
    }
}