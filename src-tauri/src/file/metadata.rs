use std::error::Error;
use std::io::Error as IoError;
use std::path::Path;

use crate::db::sqlite::Session;
use crate::util::snowflake::id;
use chrono::{DateTime, Utc};
use file_hashing::get_hash_file;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use sqlx::query;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Metadata {
    pub id: i64,
    pub file_path: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_suffix: String,
    pub added: String,
    pub created: String,
    pub modified: String,
    pub tags: String,
    pub exegesis: String,
    pub score: f32,
    pub is_del: u8,
    pub sha1: String,
    // image
    pub image_width: u32,
    pub image_height: u32,
    pub thumbnail: String,
    pub colors: String,
    pub shape: String,
    // video
    pub duration: i64,
}

impl Metadata {
    pub fn empty() -> Self {
        Metadata {
            id: -1,
            file_path: String::new(),
            file_name: String::new(),
            file_size: 0,
            file_suffix: String::new(),
            added: String::new(),
            created: String::new(),
            modified: String::new(),
            tags: String::new(),
            exegesis: String::new(),
            score: 0.0,
            is_del: 0,
            sha1: String::new(),
            // image
            image_width: 0,
            image_height: 0,
            thumbnail: String::new(),
            colors: String::new(),
            shape: String::new(),
            // video
            duration: 0,
        }
    }

    pub fn load(path: &Path) -> Self {
        let mut metadata = Self::empty();
        if let Some(extension) = path.extension() {
            if let Some(file_suffix) = extension.to_str() {
                metadata.file_suffix = file_suffix.to_lowercase();
            }
        }
        metadata
    }

    /// 解析文件元数据
    pub fn analyze_metadata(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        if let Some(file_path) = path.to_str() {
            self.file_path = file_path.to_string();
        }
        if let Some(file_name) = path.file_name() {
            if let Some(file_name) = file_name.to_str() {
                self.file_name = file_name.to_string();
            }
        }
        let file_metadata = path.metadata()?;
        self.file_size = file_metadata.len() as i64;
        let datetime: DateTime<Utc> = file_metadata.created()?.into();
        self.created = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        let datetime: DateTime<Utc> = file_metadata.modified()?.into();
        self.modified = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        self.sha1 = sha1(path)?;
        Ok(())
    }

    pub async fn save_to_db(&self) {
        let mut session = Session::new("./db/main.db");
        session.connect().await;
        if let Ok(pool) = &session.get_pool() {
            if let Ok(result) = session
                .count(
                    format!(
                        "SELECT COUNT(*) AS count FROM metadata WHERE sha1 = '{}'",
                        &self.sha1
                    )
                    .as_str(),
                )
                .await
            {
                if result.count == 0 {
                    let _ = query("INSERT INTO metadata (id, file_name, file_path, file_size, file_suffix, added, created, modified, image_width, image_height, thumbnail, tags, exegesis, score, colors, shape, duration, is_del, sha1) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                        .bind(id::<i64>())
                        .bind(&self.file_name)
                        .bind(&self.file_path)
                        .bind(&self.file_size)
                        .bind(&self.file_suffix)
                        .bind(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
                        .bind(&self.created)
                        .bind(&self.modified)
                        .bind(&self.image_width)
                        .bind(&self.image_height)
                        .bind(&self.thumbnail)
                        .bind(&self.tags)
                        .bind(&self.exegesis)
                        .bind(&self.score)
                        .bind(&self.colors)
                        .bind(&self.shape)
                        .bind(&self.duration)
                        .bind(&self.is_del)
                        .bind(&self.sha1)
                        .execute(pool)
                        .await;
                }
            }
        }
    }
}

fn sha1<P: AsRef<Path>>(path: P) -> Result<String, IoError> {
    let mut hasher = Sha1::new();
    get_hash_file(path, &mut hasher)
}
