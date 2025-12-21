use axum::{json, extract::State};
use sqlx::PgPool;
use uuid::Uuid;

use create::models::todo::{CreateTodo, Todo};

pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTodo>,
) -> Result<Todo> {
    let todo = sqlx::query_as!(
        Todo,
         r#"
            INSERT INTO todos (title)
            VALUES ($1, $2)
            RETURNING id, title, streak, last_completed
         "#,
         Uuid::new_v4(),
         payload.title
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(todo)
}

pub async fn list_todos(
    State(pool): State<PgPool>,
) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
            SELECT id, title, streak, last_completed FROM todos
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(todos)
}