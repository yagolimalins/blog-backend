use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: i64,
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let row: Result<(Uuid, String), _> =
        sqlx::query_as("SELECT id, password FROM users WHERE email = $1")
            .bind(&payload.email)
            .fetch_one(&pool)
            .await;

    let (id, hashed_password) = match row {
        Ok(user) => user,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid credentials"})),
            )
                .into_response();
        }
    };

    let parsed_hash = match PasswordHash::new(&hashed_password) {
        Ok(h) => h,
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"message": "Invalid credentials"})),
        )
            .into_response();
    }

    let claims = Claims {
        sub: id.to_string(),
        exp: Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("Valid timestamp")
            .timestamp(),
    };

    let secret = env::var("TOKEN_SECRET").expect("TOKEN_SECRET environment variable must be set");

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(token) => (StatusCode::OK, Json(json!({ "token": token }))).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
