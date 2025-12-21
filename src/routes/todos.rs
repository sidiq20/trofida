use axum::{Router, routing::{get, post}, Json, extract::{Path, State}};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};

use crate::{
    errors::AppError,
    handlers::AuthUser,
    models::todo::Todo
};
use sqlx::PgPool;

use crate::handlers::todos::*;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/todos", post(create_todo).get(list_todos))
        .with_state(pool)
}


pub async fn complete_todo(
    AuthUser { id: user_id }: AuthUser,
    Path(todo_id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> Result<Json<Todo>, AppError> {
    let today = Utc::now().date_naive();

    let mut todo = sqlx::query_as!(
        Todo,
        r#"
        SELECT ID, title, streak, last_completed
        FROM todos
        WHERE id = $1 AND user_id = $2
        "#,
        todo_id,
        user_id
    )
    .fecth_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    match todo.last_completed {
        Some(date) if date == today => {
            // already completed
        }
        Some(date) if date == today.pred() => {
            todo.streak += 1;
        }
        _ => {
            todo.streak = 1;
        }
    }

    let updated = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET streak = $1, last_completed = $2
        where id = $3
        RETURNING id, title, streak, last_completed
        "#,
        todo.streak,
        today,
        todo.id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(updated))
}
