use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::blogs::{get_blog_by_slug, get_blogs};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_blogs))
        .route("/{slug}", get(get_blog_by_slug))
}
