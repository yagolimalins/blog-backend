use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::PgPool;

use crate::handlers::users::{
    create_user, delete_user_by_username, get_user_blogs_by_username, get_user_by_username,
    get_users,
};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_users))
        .route("/{username}", get(get_user_by_username))
        .route("/", post(create_user))
        .route("/{username}", delete(delete_user_by_username))
        .route("/{username}/blogs", get(get_user_blogs_by_username))
}
