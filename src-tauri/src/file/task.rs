use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub file_path: String,
    pub file_suffix: String,
    pub status: u8,
}
