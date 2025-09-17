use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}
