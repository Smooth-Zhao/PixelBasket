use std::error::Error;
use std::io::Error as IoError;
use std::ops::Add;
use std::path::Path;

use chrono::{DateTime, Local};
use file_hashing::get_hash_file;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use sqlx::query;

use crate::db::sqlite::Session;
use crate::util::error::ErrorHandle;
use crate::util::snowflake::id;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Metadata {
    pub id: i64,
    pub full_path: String,
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
            full_path: String::new(),
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
        let mut suffix = String::new();
        if let Some(extension) = path.extension() {
            if let Some(file_suffix) = extension.to_str() {
                suffix = ".".to_string().add(file_suffix);
                metadata.file_suffix = file_suffix.to_lowercase();
            }
        }
        let mut name = String::new();
        if let Some(file_name) = path.file_name() {
            if let Some(file_name) = file_name.to_str() {
                name = file_name.to_string();
                let re = format!("{}$", suffix);
                if let Some(re) = Regex::new(re.as_str()).print_error() {
                    metadata.file_name = re.replace(file_name, "".to_string()).to_string();
                }
            }
        }
        if let Some(file_path) = path.to_str() {
            metadata.full_path = file_path.to_string();
            let re = format!("{}$", name);
            if let Some(re) = Regex::new(re.as_str()).print_error() {
                metadata.file_path = re.replace(file_path, "".to_string()).to_string();
            }
        }
        metadata
    }

    /// 解析文件元数据
    pub fn analyze_metadata(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let file_metadata = path.metadata()?;
        self.file_size = file_metadata.len() as i64;
        let datetime: DateTime<Local> = file_metadata.created()?.into();
        self.created = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        let datetime: DateTime<Local> = file_metadata.modified()?.into();
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
                        .bind(Local::now().format("%Y-%m-%d %H:%M:%S").to_string())
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

    pub async fn save_task_to_db(&self) {
        let mut session = Session::new("./db/main.db");
        session.connect().await;
        if let Ok(pool) = &session.get_pool() {
            if let Ok(result) = session
                .count(
                    format!(
                        "SELECT COUNT(*) AS count FROM task WHERE file_path = '{}'",
                        &self.full_path
                    )
                    .as_str(),
                )
                .await
            {
                if result.count == 0 {
                    let _ = query(
                        "INSERT INTO task (id, file_path, file_suffix, status) VALUES (?, ?, ?, ?)",
                    )
                    .bind(id::<i64>())
                    .bind(&self.full_path)
                    .bind(&self.file_suffix)
                    .bind(0)
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataVO {
    pub id: String,
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
    pub image_width: u32,
    pub image_height: u32,
    pub thumbnail: String,
    pub colors: String,
    pub shape: String,
    pub duration: i64,
}

impl MetadataVO {
    pub fn from(metadata: Metadata) -> Self {
        Self {
            id: metadata.id.to_string(),
            file_path: metadata.file_path,
            file_name: metadata.file_name,
            file_size: metadata.file_size,
            file_suffix: metadata.file_suffix,
            added: metadata.added,
            created: metadata.created,
            modified: metadata.modified,
            tags: metadata.tags,
            exegesis: metadata.exegesis,
            score: metadata.score,
            is_del: metadata.is_del,
            sha1: metadata.sha1,
            image_width: metadata.image_width,
            image_height: metadata.image_height,
            thumbnail: metadata.thumbnail,
            colors: metadata.colors,
            shape: metadata.shape,
            duration: metadata.duration,
        }
    }

    pub fn empty() -> Self {
        Self {
            id: "-1".to_string(),
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
            image_width: 0,
            image_height: 0,
            thumbnail: String::new(),
            colors: String::new(),
            shape: String::new(),
            duration: 0,
        }
    }
}
