use std::path::Path;
use std::time::Instant;
use std::{fs, io};

use tauri::{AppHandle, Manager};

use crate::file::Directory;

pub struct ScanJob {
    app_handle: AppHandle,
    count: i32,
}

impl ScanJob {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            count: 0,
        }
    }

    pub fn get_directory_tree(&mut self, dir: &Path) -> String {
        let start = Instant::now(); // 获取当前时间
        if let Ok(directories) = self.walk(dir) {
            self.send_count();
            let json = serde_json::to_string(&directories).unwrap();
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
                if path.is_dir() {
                    match self.walk(&path) {
                        Ok(directories) => directory.children = directories,
                        Err(e) => println!("Error reading directories: {}", e),
                    }
                    // 如果是文件，打印路径
                    // println!("【Dir】{}", path.display());
                } else {
                    self.count += 1;
                    if self.count % 1000 == 0 {
                        self.send_count();
                    }
                    // 如果是文件，打印路径
                    // println!("【File】{}", path.display());
                }
            }
            directories.push(directory);
        }
        Ok(directories)
    }

    pub fn send_count(&self) {
        let _ = self.app_handle.emit_all("app", self.count);
    }
}
