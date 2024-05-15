use dotenv::dotenv;
use tauri::Manager;

use pixel_basket::config::set_db_path;
use pixel_basket::util::error::ErrorHandle;
use pixel_basket::{basket, APP_HANDLE};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            basket::create_basket,
            basket::get_metadata,
            basket::del_metadata,
            basket::get_metadata_by_id,
            basket::get_metadata_like_path,
            basket::get_basket,
            basket::del_basket,
            basket::get_folder,
            basket::run_task
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
