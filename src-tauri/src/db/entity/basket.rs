use serde::{Deserialize, Serialize};
use sqlx::query;

use crate::db::entity::folder::Folder;
use crate::db::sqlite::Session;
use crate::util::error::ErrorHandle;
use crate::util::snowflake::id;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Basket {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasketVO {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct BasketFolder {
    pub id: i64,
    pub basket_id: i64,
    pub folder_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasketData {
    pub name: String,
    pub directories: Vec<String>,
}

impl Basket {
    pub fn new(name: String) -> Self {
        Self { id: id(), name }
    }

    pub async fn exist(&self, session: &Session) -> bool {
        if let Ok(result) = session
            .count(
                format!(
                    "SELECT COUNT(*) AS count FROM basket WHERE name = '{}'",
                    &self.name
                )
                .as_str(),
            )
            .await
        {
            return result.count > 0;
        }
        false
    }

    pub async fn save(&self, session: &Session) {
        if let Some(pool) = session.as_pool().print_error() {
            query("INSERT INTO basket (id, name) VALUES (?, ?)")
                .bind(&self.id)
                .bind(&self.name)
                .execute(pool)
                .await
                .print_error();
        }
    }

    pub async fn save_folder(&self, directories: &Vec<String>, session: &Session) {
        if let Some(basket) = session
            .select_one_as::<Basket>(&format!(
                "SELECT * FROM basket WHERE name = '{}'",
                self.name
            ))
            .await
            .print_error()
        {
            let str = directories
                .iter()
                .map(|v| format!("'{v}'"))
                .collect::<Vec<String>>()
                .join(",");
            if let Some(folders) = session
                .select_as::<Folder>(&format!("SELECT * FROM folder WHERE path IN ({str})"))
                .await
                .print_error()
            {
                if let Some(pool) = session.as_pool().print_error() {
                    for folder in folders {
                        let basket_folder = BasketFolder::new(basket.id, folder.id);
                        if !basket_folder.exist(&session).await {
                            query(
                            "INSERT INTO basket_folder (id, basket_id, folder_id) VALUES (?, ?, ?)",
                        )
                        .bind(&basket_folder.id)
                        .bind(&basket_folder.basket_id)
                        .bind(&basket_folder.folder_id)
                        .execute(pool)
                        .await
                        .print_error();
                        }
                    }
                }
            }
        }
    }
}

impl BasketFolder {
    pub fn new(basket_id: i64, folder_id: i64) -> Self {
        Self {
            id: id(),
            basket_id,
            folder_id,
        }
    }

    pub async fn exist(&self, session: &Session) -> bool {
        if let Ok(result) = session
            .count(
                format!(
                    "SELECT COUNT(*) AS count FROM basket_folder WHERE basket_id = {} AND folder_id = {}",
                    &self.basket_id, &self.folder_id
                )
                .as_str(),
            )
            .await
        {
            return result.count > 0;
        }
        false
    }
}

impl BasketVO {
    pub fn from(basket: Basket) -> Self {
        Self {
            id: basket.id.to_string(),
            name: basket.name,
        }
    }

    pub fn empty() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
        }
    }
}
