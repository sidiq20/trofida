use sqlx::PgPool;
use uuid::Uuid;
use crate::domains::todo::models::{Todo, CreateTodo, TodoFolder, CreateTodoFolder, TodoStatus, FolderStatus};

pub struct TodoService;

impl TodoService {
    pub async fn create_folder(
        pool: &PgPool,  
        user_id: Uuid,
        input: CreateTodoFolder,
    ) -> Result<TodoFolder, sqlx::Error> {
        let folder = sqlx::query_as!(
            TodoFolder,
            r#"
            INSERT INTO todo_folders (user_id, name)
            VALUES ($1, $2)
            RETURNING id, user_id, name, status as "status!: _", created_at as "created_at!", updated_at as "updated_at!"
            "#,
            user_id,
            input.name
        )
        .fetch_one(pool)
        .await?;

        Ok(folder)
    }

    pub async fn create_todo(
        pool: &PgPool,
        user_id: Uuid,
        input: CreateTodo,
    ) -> Result<Todo, sqlx::Error> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
            INSERT INTO todos (user_id, folder_id, title, streak_required)
            VALUES ($1, $2, $3, $4)
            RETURNING id as "id!", user_id as "user_id!", folder_id, title, streak, last_completed, streak_required, created_at as "created_at!", status as "status!:_"
            "#,
            user_id,
            input.folder_id,
            input.title,
            input.streak_required
        )
        .fetch_one(pool)
        .await?;

        Ok(todo)
    }

    pub async fn get_folder_for_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<TodoFolder>, sqlx::Error> {
        sqlx::query_as!(
            TodoFolder,
            r#"SELECT id, user_id, name, status as "status!: _", created_at as "created_at!", updated_at as "updated_at!" FROM todo_folders WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(pool)
        .await
    }
}