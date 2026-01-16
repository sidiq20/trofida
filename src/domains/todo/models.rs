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

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "folder_status", rename_all = "snake_case")]
pub enum FolderStatus {
    Active,
    Archived,
}

#[derive(SimpleObject, FromRow, Clone)]
pub struct TodoFolder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub status: FolderStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(InputObject)]
pub struct CreateTodoFolder {
    pub name: String,
    pub status: FolderStatus,
}

#[derive(SimpleObject, FromRow, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub folder_id: Option<Uuid>,
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
    pub folder_id: Option<Uuid>,
    pub streak_required: i32,
}
