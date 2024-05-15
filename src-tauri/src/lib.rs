pub mod basket;
pub mod config;
pub mod db;
pub mod file;
pub mod util;

use core::result::Result as CoreResult;
use once_cell::sync::Lazy;
use std::error::Error;
use std::sync::Mutex;
use tauri::AppHandle;

pub type Result<T> = CoreResult<T, Box<dyn Error>>;

pub static APP_HANDLE: Lazy<Mutex<Option<AppHandle>>> = Lazy::new(|| Mutex::new(None));
