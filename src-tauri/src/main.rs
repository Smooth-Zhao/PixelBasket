// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use pixel_basket::file;
use std::env;
use std::path::Path;
use std::time::Instant;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_directory_tree(path: &str) -> String {
    let start = Instant::now(); // 获取当前时间
    let path = Path::new(path);
    let result = file::get_directory_tree(&path);

    let end = Instant::now(); // 获取当前时间
    let duration = end - start; // 计算运行时间
    println!("代码运行时间为： {:?} 秒", duration.as_secs());

    result
}

fn main() {
    dotenv().ok();
    // 现在你可以使用 env::var() 来读取环境变量
    let port: String = env::var("DATABASE_URL").expect("PORT environment variable is not set");
    println!("Listening on port: {}", port);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_directory_tree])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
