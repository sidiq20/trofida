use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub streak: i32,
    pub last_completed: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
}