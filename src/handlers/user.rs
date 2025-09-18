use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{Error, PgPool, query, query_as};
use uuid::Uuid;

use crate::models::{
    blog::Blog,
    user::{CreateUser, User, UserResponse},
};

pub async fn get_users(State(pool): State<PgPool>) -> impl IntoResponse {
    match query_as::<_, UserResponse>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
    {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_user_by_id(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match query_as::<_, UserResponse>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
    {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    match query_as::<_, User>("SELECT * FROM users WHERE username = $1 OR email = $2")
        .bind(&payload.username)
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(_)) => return StatusCode::CONFLICT.into_response(),
        Ok(None) => {}
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let result = query_as::<_, UserResponse>(
        r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, created_at
        "#,
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(password)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_user(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    match query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_user_blogs(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match query_as::<_, Blog>("SELECT * FROM blogs WHERE user_id = $1")
        .bind(id)
        .fetch_all(&pool)
        .await
    {
        Ok(blogs) if blogs.is_empty() => StatusCode::NOT_FOUND.into_response(),
        Ok(blogs) => (StatusCode::OK, Json(blogs)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
