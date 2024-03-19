use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, io};

#[derive(Serialize, Deserialize)]
pub struct Directory {
    path: String,
    children: Vec<Directory>,
}

pub fn get_directory_tree(dir: &Path) -> String {
    match walk(dir) {
        Ok(directories) => {
            let json = serde_json::to_string(&directories).unwrap();
            return json;
        }
        _ => {}
    };
    return String::new();
}

fn walk(dir: &Path) -> io::Result<Vec<Directory>> {
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
                    match walk(&path) {
                        Ok(directories) => directory.children = directories,
                        Err(e) => println!("Error reading directories: {}", e),
                    }
                    // 如果是文件，打印路径
                    // println!("【Dir】{}", path.display());
                } else {
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
