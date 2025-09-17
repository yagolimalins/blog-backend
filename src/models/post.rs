use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub blog_id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
}
