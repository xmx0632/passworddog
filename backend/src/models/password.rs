use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub username: String,
    pub encrypted_password: String,
    pub website: Option<String>,
    pub notes: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
