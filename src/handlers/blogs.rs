use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{PgPool, Postgres, query, query_as};

use crate::models::blog::{Blog, BlogResponse};

pub async fn get_blogs(State(pool): State<PgPool>) -> impl IntoResponse {
    match query_as::<Postgres, BlogResponse>(
        r#"
        SELECT
       	    *
    	FROM
    	    blogs b
        JOIN users u on
    	    b.user_id = u.id
        "#,
    )
    .fetch_all(&pool)
    .await
    {
        Ok(blogs) => (StatusCode::OK, Json(blogs)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_blog_by_slug(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match query_as::<Postgres, BlogResponse>(
        r#"
        SELECT
       	    *
    	FROM
    	    blogs b
        JOIN users u on
    	    b.user_id = u.id
    	WHERE
    	    b.slug = $1
        "#,
    )
    .bind(&slug)
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(blog)) => (StatusCode::OK, Json(blog)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_blog_by_slug(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match query_as::<Postgres, Blog>("SELECT * FROM blogs WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(_)) => (),
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    match query("DELETE FROM blogs WHERE slug = $1")
        .bind(&slug)
        .execute(&pool)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
