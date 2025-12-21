use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::auth::*;

pub fn routes(pool: PgPool) -> ROuter {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .with_state(pool)
}

use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::request::Parts,
};

use uuid::Uuid;

use crate::{utils::jwt::decode_jwt, erros::AppError};

pub struct AuthUser {
    pub id: uuid
}

#[async_trait]
impl<s> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Regection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &S,
    ) -> Result<Self, Self::Regection> {
        let auth_header = parts 
            .headers 
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token = auth_header.replace("Bearer ", "");
        let claims = decode_jwt(&token).map_err(|_| AppError::Authorization)?;

        Ok(AuthUser { id: claim.sub })
    }
}