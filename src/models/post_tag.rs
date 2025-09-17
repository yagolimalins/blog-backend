use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct PostTag {
    pub post_id: Uuid,
    pub tag_id: Uuid,
}
