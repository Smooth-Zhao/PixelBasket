use serde::{Deserialize, Serialize};
use std::vec;
use tokio::sync::mpsc::channel;

use crate::file::image_scanner::ImageScanner;
use crate::file::model_scanner::ModelScanner;
use crate::file::scan::ScanJob;

#[derive(Debug, Serialize, Deserialize)]
pub struct Basket {
    name: String,
    directories: Vec<String>,
}

#[tauri::command]
pub fn create_basket(basket: Basket, _app_handle: tauri::AppHandle) -> &'static str {
    let (tx, mut rx) = channel::<String>(16);
    let mut scan = ScanJob::new();
    scan.add_scanners(vec![ImageScanner::wrap(), ModelScanner::wrap()]);
    scan.run(basket.directories, tx.clone());
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("{}", msg);
        }
    });
    "OK"
}
