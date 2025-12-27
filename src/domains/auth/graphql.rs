use async_graphql::{Object, Context, Error};
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::jwt::create_jwt;
use crate::utils::password::{hash_password, verify_password};
use super::models::{AuthPayload, AuthResponse};

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn register(&self, ctx: &Context<'_>, input: AuthPayload) -> async_graphql::Result<AuthResponse> {
        let pool = ctx.data::<PgPool>()?;
        
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
}
