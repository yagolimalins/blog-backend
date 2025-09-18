use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Blog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}
