use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use uuid::Uuid;
use crate::auth::AuthUser;

use crate::{
    utils::jwt::decode_jwt,
    errors::AppError,
};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        let claims = decode_jwt(token)
            .map_err(|_| AppError::Unauthorized)?;

        Ok(AuthUser { id: claims.sub })
    }
}
