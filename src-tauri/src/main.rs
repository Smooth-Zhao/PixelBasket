use std::env;
use std::path::Path;
use std::time::Instant;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use dotenv::dotenv;
use tauri::Manager;

use pixel_basket::basket;
// use pixel_basket::file;
use pixel_basket::file::*;

// use once_cell::sync::Lazy;
// use std::path::Path;
// use std::time::Instant;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_directory_files(path: &str) -> String {
    let start = Instant::now(); // 获取当前时间
    let path = Path::new(path);
    let result = get_directory_tree(&path);

    let end = Instant::now(); // 获取当前时间
    let duration = end - start; // 计算运行时间
    println!("代码运行时间为： {:?} 秒", duration.as_secs());

    result
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    // 现在你可以使用 env::var() 来读取环境变量
    let port: String = env::var("DATABASE_URL").expect("PORT environment variable is not set");
    println!("Listening on port: {}", port);

    tauri::Builder::default()
        .setup(move |app| {
            // 设置 AppHandle 的值
            let mut handle = APP_HANDLE.lock().unwrap();
            *handle = Some(app.app_handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            basket::create_basket,
            basket::get_metadata,
            basket::del_metadata,
            get_directory_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
