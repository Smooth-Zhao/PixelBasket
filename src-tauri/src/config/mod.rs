use crate::debug;
use tauri::AppHandle;

pub static mut DB: String = String::new();

pub fn get_db_path() -> &'static str {
    unsafe { DB.as_str() }
}

pub fn set_db_path(app_handle: AppHandle) {
    if let Some(resource) = app_handle.path_resolver().resolve_resource("db/main.db") {
        if let Some(path) = resource.to_str() {
            let path = path.to_string();
            debug!("DB path: {}", &path);
            unsafe { DB = path }
        }
    }
}
