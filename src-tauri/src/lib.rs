use core::result::Result as CoreResult;
use std::error::Error;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use tauri::{AppHandle, Manager};

use crate::config::set_db_path;
use crate::handler::basket::*;
use crate::handler::config::*;
use crate::handler::folder::*;
use crate::handler::metadata::*;
use crate::handler::task::*;
use crate::util::error::ErrorHandle;

pub mod config;
pub mod data;
pub mod scanner;
pub mod handler;
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
        .setup(move |app| {
            // 设置 AppHandle 的值
            let mut handle = APP_HANDLE.lock().unwrap();
            *handle = Some(app.app_handle());
            set_db_path(app.app_handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .print_error();
}
