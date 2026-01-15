use uuid::Uuid;
use async_graphql::{SimpleObject, InputObject, Enum};
use chrono::{NaiveDateTime, NaiveDate};

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "folder_status", rename_all = "snake_case")]
pub enum FolderStatus {
    Active,
    Archived,
}

#[derive(SimpleObject, Clone)]
pub struct TodoFolder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}