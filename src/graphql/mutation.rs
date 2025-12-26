use async_graphql::{Object, Context, InputObject, SimpleObject, Error};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::todo::{Todo, CreateTodo, TodoStatus};
use crate::utils::jwt::create_jwt;
use crate::utils::password::{hash_password, verify_password};

#[derive(InputObject)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct AuthResponse {
    pub token: String,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // --- AUTH ---

    async fn register(&self, ctx: &Context<'_>, input: AuthPayload) -> async_graphql::Result<AuthResponse> {
        let pool = ctx.data::<PgPool>()?;
        
        // In a real app, use argon2 here. For this refactor, I'll match the import style 
        // or check if 'argon2' crate is available.  
        // Assuming we need to hash password. 
        // Based on handlers/auth.rs, it used `argon2::hash_password`.
        // If dependencies are missing, I'll add them or stick to basic flow.
        // Using a placeholder hash for now to ensure compilation if imports are tricky, 
        // but let's try to do it right if possible.
        // The previous auth.rs had `use argon2...`.
        
        // Simulating hash for 'demo' if argon2 isn't easily accessible without adding deps,
        // BUT handlers/auth.rs implied it was there.
        // Let's assume standard behavior:
        
        let user_id = Uuid::new_v4();
        let password_hash = hash_password(&input.password)
            .map_err(|e| Error::new(format!("Failed to hash password: {}", e)))?; 

        sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash)
            VALUES ($1, $2, $3)
            "#,
            user_id,
            input.email,
            password_hash,
        )
        .execute(pool)
        .await
        .map_err(|e| Error::new(e.to_string()))?;

        let token = create_jwt(user_id)?;
        
        Ok(AuthResponse { token })
    }

    async fn login(&self, ctx: &Context<'_>, input: AuthPayload) -> async_graphql::Result<AuthResponse> {
        let pool = ctx.data::<PgPool>()?;

        let user = sqlx::query!(
            r#"
            SELECT id, password_hash
            FROM users
            WHERE email = $1
            "#,
            input.email,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::new(e.to_string()))?
        .ok_or_else(|| Error::new("Invalid credentials"))?;

        let valid = verify_password(&input.password, &user.password_hash)
            .map_err(|e| Error::new(format!("Password verification error: {}", e)))?;

        if !valid {
            return Err(Error::new("Invalid credentials"));
        }
         
        let token = create_jwt(user.id)?;
        
        Ok(AuthResponse { token })
    }

    // --- TODOS ---

    async fn create_todo(&self, ctx: &Context<'_>, input: CreateTodo) -> async_graphql::Result<Todo> {
        let pool = ctx.data::<PgPool>()?;
        let user = ctx.data::<crate::utils::authuser::AuthUser>()
            .map_err(|_| Error::new("Unauthorized: Please log in"))?;
        
        let todo = sqlx::query_as!(
            Todo,
            r#"
            INSERT INTO todos (id, user_id, title, streak_required, streak)
            VALUES ($1, $2, $3, $4, 0)
            RETURNING id, user_id as "user_id!", title, streak, last_completed, streak_required, created_at as "created_at!", status as "status!: TodoStatus"
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
        
        // Verify ownership/existence first or do it in one query
        let todo = sqlx::query_as!(
            Todo,
            r#"UPDATE todos 
            SET last_completed = NOW(), streak = streak + 1 
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id as "user_id!", title, streak, last_completed, streak_required, created_at as "created_at!", status as "status!: TodoStatus""#,
            id,
            user.id
        )
        .fetch_optional(pool) // Use fetch_optional to handle not found vs error
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
