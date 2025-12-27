use async_graphql::{SimpleObject, InputObject};

#[derive(InputObject)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct AuthResponse {
    pub token: String,
}
