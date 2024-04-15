use std::vec;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::channel;

use crate::db::sqlite::Session;
use crate::file::image_scanner::ImageScanner;
use crate::file::metadata::Metadata;
use crate::file::model_scanner::ModelScanner;
use crate::file::scan::ScanJob;
use crate::util::error::ErrorHandle;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    size: usize,
    current: usize,
}

#[tauri::command]
pub async fn get_metadata(page: Page) -> Vec<Metadata> {
    let mut session = Session::new("./db/main.db");
    session.connect().await;
    let sql = format!(
        "SELECT * FROM metadata WHERE is_del = 0 LIMIT {},{}",
        page.current, page.size
    );
    if let Some(metadata) = session.select_as::<Metadata>(&sql).await.print_error() {
        return metadata;
    }
    Vec::new()
}

#[tauri::command]
pub async fn del_metadata(id: i64) -> bool {
    let mut session = Session::new("./db/main.db");
    session.connect().await;
    let sql = format!("UPDATE metadata SET is_del = 1 WHERE id = {}", id);
    session.execute(&sql).await.print_error().is_some()
}
