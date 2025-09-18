use axum::http::{StatusCode, request::Parts};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub struct AuthUser(pub Claims);

impl<S> axum::extract::FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".into(),
            ))?;

        let token = auth_header.strip_prefix("Bearer ").ok_or((
            StatusCode::UNAUTHORIZED,
            "Invalid Authorization scheme".into(),
        ))?;

        let secret = env::var("TOKEN_SECRET").map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "TOKEN_SECRET not set".into(),
            )
        })?;

        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".into()))?;

        if decoded.claims.exp < Utc::now().timestamp() {
            return Err((StatusCode::UNAUTHORIZED, "Token expired".into()));
        }

        Ok(AuthUser(decoded.claims))
    }
}
