use crate::db::sqlite::Session;
use crate::util::error::ErrorHandle;
use serde::{Deserialize, Serialize};
use sqlx::query;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Config {
    pub key: String,
    pub value: String,
}

impl Config {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }

    pub async fn save(&self, session: &Session) {
        if let Some(pool) = session.as_pool().print_error() {
            query("INSERT INTO config (key, value) VALUES (?, ?)")
                .bind(&self.key)
                .bind(&self.value)
                .execute(pool)
                .await
                .print_error();
        }
    }

    pub async fn update(&self, session: &Session) {
        if let Some(pool) = session.as_pool().print_error() {
            query("UPDATE config SET value = ? WHERE key = ?")
                .bind(&self.value)
                .bind(&self.key)
                .execute(pool)
                .await
                .print_error();
        }
    }

    pub async fn get(key: String, session: &Session) -> Option<Self> {
        let sql = format!("SELECT * FROM config WHERE key = {key}");
        session.select_one_as::<Config>(&sql).await.print_error()
    }

    pub async fn delete(key: String, session: &Session) {
        if let Some(pool) = session.as_pool().print_error() {
            query("DELETE FROM config WHERE key =  ?")
                .bind(key)
                .execute(pool)
                .await
                .print_error();
        }
    }
}
