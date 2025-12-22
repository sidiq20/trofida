use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{NaiveDate, NaiveDateTime};
use async_graphql::{SimpleObject, InputObject};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Type)]
#[sqlx(type_name = "todo_status", rename_all = "lowercase")]
pub enum TodoStatus {
    Active,
    Paused,
    Completed,
}


#[derive(Debug, Serialize, SimpleObject, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}
#[derive(Serialize, SimpleObject, FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub streak: i32,
    pub last_completed: Option<NaiveDate>,
    pub streak_required: i32,
    pub status: TodoStatus,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, InputObject)]
pub struct CreateTodo {
    pub title: String,
    pub streak_required: i32,
    pub status: TodoStatus,
}

#[derive(Debug, Serialize, SimpleObject, FromRow)]
pub struct TodoCompletion {
    pub id: Uuid,
    pub todo_id: Uuid,
    pub completed_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, SimpleObject, FromRow)]
pub struct Commitment {
    pub id: Uuid,
    pub todo_id: Uuid,
    pub stake_amount: i64,
    pub vault_pubkey: String,
    pub start_date: chrono::NaiveDate,
    pub last_checkin: Option<chrono::NaiveDate>,
    pub streak_current: i32,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
}