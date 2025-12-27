use async_graphql::{SimpleObject, InputObject, Enum};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{NaiveDateTime, NaiveDate};

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "todo_status", rename_all = "snake_case")]
pub enum TodoStatus {
    Open,
    Completed,
}

#[derive(SimpleObject, FromRow, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub streak: i32,
    pub last_completed: Option<NaiveDate>,
    pub streak_required: i32,
    pub created_at: NaiveDateTime,
    pub status: TodoStatus,
}

#[derive(InputObject)]
pub struct CreateTodo {
    pub title: String,
    pub streak_required: i32,
}
