use core::result::Result as CoreResult;
use std::error::Error;
use std::fs;
use std::sync::Mutex;

use dirs::data_local_dir;
use once_cell::sync::Lazy;
use tauri::{AppHandle, Manager};

use crate::config::{APP_CONFIG, APP_NAME, get_app_config, INIT_DB, set_app_config};
use crate::handler::basket::*;
use crate::handler::config::*;
use crate::handler::folder::*;
use crate::handler::metadata::*;
use crate::handler::task::*;
use crate::util::error::ErrorHandle;

pub mod config;
pub mod data;
pub mod handler;
pub mod scanner;
pub mod util;

pub type Result<T> = CoreResult<T, Box<dyn Error>>;

pub static APP_HANDLE: Lazy<Mutex<Option<AppHandle>>> = Lazy::new(|| Mutex::new(None));

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_basket,
            get_basket,
            del_basket,
            get_metadata,
            del_metadata,
            get_metadata_by_id,
            get_metadata_like_path,
            get_folder,
            run_task,
            save_config,
            update_config,
            get_config,
            del_config,
        ])
        .setup(move |app| init(&app.app_handle()))
        .run(tauri::generate_context!())
        .print_error();
}

fn init(app_handle: &AppHandle) -> Result<()> {
    if let Some(mut dir) = data_local_dir() {
        dir.push(APP_NAME);
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
            debug!("INIT ==> 创建数据目录: {}", dir.clone().display());
        }
        let mut conf_path_buf = dir.clone();
        conf_path_buf.push(APP_CONFIG);
        let mut config = get_app_config(&conf_path_buf)?;
        if config.path.db.is_empty() {
            let mut db_path_buf = dir.clone();
            db_path_buf.push(".db");
            if let Some(resource) = app_handle.path_resolver().resolve_resource(INIT_DB) {
                fs::copy(resource, &db_path_buf)?;
                config.path.db = db_path_buf.to_string_lossy().to_string();
                set_app_config(&conf_path_buf, &config)?;
                debug!("INIT ==> 初始化数据库: {}", config.path.db);
            }
        }
        if config.path.cache.is_empty() {
            let mut cache_path_buf = dir.clone();
            cache_path_buf.push("cache");
            fs::create_dir(&cache_path_buf)?;
            config.path.cache = cache_path_buf.to_string_lossy().to_string();
            set_app_config(&conf_path_buf, &config)?;
            debug!(
                "INIT ==> 创建缓存目录: {}",
                cache_path_buf.clone().display()
            );
        }
    }
    Ok(())
}
