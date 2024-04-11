use std::path::Path;

use crate::file::image_scanner::ImageScanner;
use crate::file::model_scanner::ModelScanner;
use serde::{Deserialize, Serialize};

use crate::file::scan::ScanJob;

#[derive(Debug, Serialize, Deserialize)]
pub struct Basket {
    name: String,
    directories: Vec<String>,
}

#[tauri::command]
pub fn create_basket(basket: Basket, _app_handle: tauri::AppHandle) -> &'static str {
    for dir in basket.directories {
        tokio::spawn(async move {
            let mut scan = ScanJob::new();
            scan.add_scanner(ImageScanner::wrap());
            scan.add_scanner(ModelScanner::wrap());
            scan.load_dir(Path::new(dir.as_str()));
            scan.run_scanner();
        });
    }
    "OK"
}
