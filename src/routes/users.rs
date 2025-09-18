use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::PgPool;

use crate::handlers::user::{create_user, delete_user, get_user_blogs, get_user_by_id, get_users};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_users))
        .route("/{id}", get(get_user_by_id))
        .route("/", post(create_user))
        .route("/{id}", delete(delete_user))
        .route("/{id}/blogs", get(get_user_blogs))
}
