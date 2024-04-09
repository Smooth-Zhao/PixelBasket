use std::error::Error;
use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::{fs, io};

use tauri::{AppHandle, Manager};

use crate::file::Directory;

pub trait Scanner {
    fn scan(&self, path: &Path) -> Result<bool, Box<dyn Error>>;
}

pub struct ScanJob {
    app_handle: AppHandle,
    count: i32,
    send_time: u128,
    scanners: Vec<Box<dyn Scanner>>,
}

impl ScanJob {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            count: 0,
            send_time: 0,
            scanners: Vec::new(),
        }
    }

    pub fn add_scanner(&mut self, scanner: Box<dyn Scanner>) {
        self.scanners.push(scanner);
    }

    pub fn get_directory_tree(&mut self, dir: &Path) -> String {
        let start = Instant::now(); // 获取当前时间
        if let Ok(directories) = self.walk(dir) {
            self.send_count(false);
            let _json = serde_json::to_string(&directories).unwrap();
            let end = Instant::now(); // 获取当前时间
            let duration = end - start; // 计算运行时间
            println!(
                "【{:?}】共{}个文件,代码运行时间为{:?}秒",
                dir.as_os_str(),
                self.count,
                duration.as_secs()
            );
        };
        return String::new();
    }

    fn walk(&mut self, dir: &Path) -> io::Result<Vec<Directory>> {
        let mut directories = Vec::new();

        if let Ok(entries) = fs::read_dir(dir) {
            // 如果是文件夹，递归调用visit_dirs
            let mut directory = Directory {
                path: dir.to_string_lossy().into_owned(),
                children: Vec::new(),
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let path = path.as_path();
                if path.is_dir() && !is_hidden(path) {
                    match self.walk(path) {
                        Ok(directories) => directory.children = directories,
                        Err(e) => println!("Error reading directories: {}", e),
                    }
                } else if path.is_file()
                    && self
                        .scanners
                        .iter()
                        .map(|v| v.scan(path).is_ok_and(|v| v))
                        .collect::<Vec<bool>>()
                        .iter()
                        .any(|v| *v)
                {
                    self.count += 1;
                    self.send_count(true)
                }
            }
            directories.push(directory);
        }
        Ok(directories)
    }

    pub fn send_count(&mut self, as_interval: bool) {
        if as_interval {
            if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
                let now = duration.as_millis();
                if self.send_time == 0 || now - self.send_time > 41 {
                    let _ = self.app_handle.emit_all("app", self.count);
                    self.send_time = now;
                }
            }
        } else {
            let _ = self.app_handle.emit_all("app", self.count);
        }
    }
}

pub fn is_hidden(path: &Path) -> bool {
    if let Some(file_name) = path.file_name() {
        if let Some(file_name) = file_name.to_str() {
            if file_name.starts_with('.') {
                return true;
            }
        }
    }
    false
}
