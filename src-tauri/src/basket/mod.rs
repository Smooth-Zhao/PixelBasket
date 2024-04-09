use std::path::Path;

use crate::file::image_scanner::ImageScanner;
use serde::{Deserialize, Serialize};

use crate::file::scan::ScanJob;

#[derive(Debug, Serialize, Deserialize)]
pub struct Basket {
    name: String,
    directories: Vec<String>,
}

#[tauri::command]
pub async fn create_basket(basket: Basket, app_handle: tauri::AppHandle) -> &'static str {
    // println!("{:?}",basket);
    for x in basket.directories.iter() {
        let string = x.clone();
        let handle = app_handle.clone();
        tokio::spawn(async move {
            let mut scan = ScanJob::new(handle);
            scan.add_scanner(ImageScanner::wrap());
            scan.get_directory_tree(Path::new(string.as_str()));
        });
    }
    "OK"
}
