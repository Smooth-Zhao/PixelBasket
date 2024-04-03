use std::{fs, io};
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

pub mod scan;
pub mod image_scanner;

#[derive(Serialize, Deserialize)]
pub struct Directory {
    path: String,
    children: Vec<Directory>,
}

pub fn get_directory_tree(dir: &Path) -> String {
    let start = Instant::now(); // 获取当前时间
    let mut count = 0;

    match walk(dir, &mut count) {
        Ok(directories) => {
            let json = serde_json::to_string(&directories).unwrap();
            let end = Instant::now(); // 获取当前时间
            let duration = end - start; // 计算运行时间
            println!(
                "【{:?}】共{}个文件,代码运行时间为{:?}秒",
                dir.as_os_str(),
                count,
                duration.as_secs()
            );
        }
        _ => {}
    };
    return String::new();
}

pub static APP_HANDLE: Lazy<Mutex<Option<AppHandle>>> = Lazy::new(|| Mutex::new(None));
fn walk(dir: &Path, count: &mut i32) -> io::Result<Vec<Directory>> {
    let mut directories = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        // 如果是文件夹，递归调用visit_dirs
        let mut directory = Directory {
            path: dir.to_string_lossy().into_owned(),
            children: Vec::new(),
        };
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    match walk(&path, count) {
                        Ok(directories) => directory.children = directories,
                        Err(e) => println!("Error reading directories: {}", e),
                    }
                    // 如果是文件，打印路径
                    // println!("【Dir】{}", path.display());
                } else {
                    *count += 1;
                    // if let Ok(mut app) = APP_HANDLE.lock() {
                    //     let option = app.as_mut();
                    //     if let Some(handle) = option{
                    //         handle.emit_all("app", count.clone());
                    //     }
                    // }
                    // let mut x = APP_HANDLE.lock().unwrap();
                    // let app_handle: &AppHandle = x.as_mut().expect("AppHandle is None");
                    // app_handle.emit_all("app", count.clone()).expect("TODO: panic message");
                    // directory.files.push(path.to_string_lossy().into_owned())
                    // 如果是文件，打印路径
                    // println!("【File】{}", path.display());
                }
            }
        }
        directories.push(directory);
    }
    Ok(directories)
}
