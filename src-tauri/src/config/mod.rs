use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

use dirs::data_local_dir;
use serde::{Deserialize, Serialize};

use crate::Result;

pub const INIT_DB: &'static str = "db/main.db";
pub const APP_CONFIG: &'static str = "conf.toml";
pub const APP_NAME: &'static str = "pixel-basket";

pub fn get_db_path() -> String {
    if let Some(mut dir) = data_local_dir() {
        dir.push(APP_NAME);
        dir.push(APP_CONFIG);
        if let Ok(config) = get_app_config(&dir) {
            return config.path.db;
        }
    }
    String::new()
}

pub fn get_cache_path() -> String {
    if let Some(mut dir) = data_local_dir() {
        dir.push(APP_NAME);
        dir.push(APP_CONFIG);
        if let Ok(config) = get_app_config(&dir) {
            return config.path.cache;
        }
    }
    String::new()
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub path: Path,
}

impl AppConfig {
    pub fn empty() -> Self {
        Self {
            path: Path::empty(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Path {
    pub db: String,
    pub cache: String,
}

impl Path {
    pub fn empty() -> Self {
        Self {
            db: String::new(),
            cache: String::new(),
        }
    }
}

pub fn get_app_config<P: AsRef<std::path::Path>>(path: P) -> Result<AppConfig> {
    if !path.as_ref().exists() {
        File::create(path.as_ref())?;
        return Ok(AppConfig::empty());
    }
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config: AppConfig = toml::from_str(&content)?;
    Ok(config)
}

pub fn set_app_config<P: AsRef<std::path::Path>>(path: P, config: &AppConfig) -> Result<()> {
    let mut file = if path.as_ref().exists() {
        OpenOptions::new().write(true).open(path)?
    } else {
        File::create(path)?
    };
    let content = toml::to_string(config)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(content.as_bytes())?;
    file.set_len(content.len() as u64)?;
    Ok(())
}
