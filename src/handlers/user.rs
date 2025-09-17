use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::{Error, PgPool, query_as};

use crate::models::user::User;

pub async fn get_users(State(pool): State<PgPool>) -> impl IntoResponse {
    let result = query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await;
    match result {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_user(
    State(pool): State<PgPool>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let result = query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(&pool)
        .await;
    match result {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
