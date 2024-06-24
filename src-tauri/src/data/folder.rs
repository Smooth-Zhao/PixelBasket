use std::path::Path;

use serde::{Deserialize, Serialize};
use sqlx::query;

use crate::util::error::ErrorHandle;
use crate::util::snowflake::id;
use crate::util::sqlite::Session;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Folder {
    pub id: i64,
    pub pid: i64,
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderVO {
    pub id: String,
    pub pid: String,
    pub name: String,
    pub path: String,
}

impl Folder {
    pub fn new(path: &Path, pid: i64) -> Self {
        Self {
            id: id(),
            pid,
            name: match path.file_name() {
                None => String::new(),
                Some(str) => match str.to_str() {
                    None => String::new(),
                    Some(str) => str.to_string(),
                },
            },
            path: match path.to_str() {
                None => String::new(),
                Some(str) => str.to_string(),
            },
        }
    }

    pub async fn exist(&self, session: &Session) -> bool {
        if let Ok(result) = session
            .count(
                format!(
                    "SELECT COUNT(*) AS count FROM folder WHERE path = '{}'",
                    &self.path
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
            query("INSERT INTO folder (id, pid, name, path) VALUES (?, ?, ?, ?)")
                .bind(&self.id)
                .bind(&self.pid)
                .bind(&self.name)
                .bind(&self.path)
                .execute(pool)
                .await
                .print_error();
        }
    }

    pub async fn update(&self, session: &Session) {
        if let Some(pool) = session.as_pool().print_error() {
            query("UPDATE folder SET pid = ?, name = ?, path = ? WHERE id = ?")
                .bind(&self.pid)
                .bind(&self.name)
                .bind(&self.path)
                .bind(&self.id)
                .execute(pool)
                .await
                .print_error();
        }
    }

    pub async fn get_by_path(session: &Session, path: &str) -> Option<Self> {
        if let Ok(result) = session
            .count(
                format!(
                    "SELECT COUNT(*) AS count FROM folder WHERE path = '{}'",
                    path
                )
                    .as_str(),
            )
            .await
        {
            if result.count > 0 {
                return session
                    .select_one_as::<Self>(&format!("SELECT * FROM folder WHERE path = '{path}'"))
                    .await
                    .print_error();
            }
        };
        None
    }
}

impl FolderVO {
    pub fn from(folder: Folder) -> Self {
        Self {
            id: folder.id.to_string(),
            pid: folder.pid.to_string(),
            name: folder.name,
            path: folder.path,
        }
    }

    pub fn empty() -> Self {
        Self {
            id: String::new(),
            pid: String::new(),
            name: String::new(),
            path: String::new(),
        }
    }
}
