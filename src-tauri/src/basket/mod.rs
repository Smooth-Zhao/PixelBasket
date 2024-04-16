use std::vec;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::channel;

use crate::db::sqlite::Session;
use crate::file::image_scanner::ImageScanner;
use crate::file::metadata::{Metadata, MetadataVO};
use crate::file::model_scanner::ModelScanner;
use crate::file::scan::{ScanJob, ScanMsg};
use crate::file::video_scanner::VideoScanner;
use crate::util::error::ErrorHandle;

#[derive(Debug, Serialize, Deserialize)]
pub struct Basket {
    name: String,
    directories: Vec<String>,
}

#[tauri::command]
pub fn create_basket(basket: Basket, _app_handle: tauri::AppHandle) -> &'static str {
    let (tx, rx) = channel::<ScanMsg>(16);
    let mut scan = ScanJob::new(tx);
    scan.add_scanners(vec![
        ImageScanner::wrap(),
        ModelScanner::wrap(),
        VideoScanner::wrap(),
    ]);
    scan.monitor_async(rx);
    scan.run_async(basket.directories);
    "OK"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    size: usize,
    current: usize,
}

#[tauri::command]
pub async fn get_metadata() -> Vec<MetadataVO> {
    let mut session = Session::new("./db/main.db");
    session.connect().await;
    if let Some(metadata) = session
        .select_as::<Metadata>("SELECT * FROM metadata WHERE is_del = 0")
        .await
        .print_error()
    {
        return metadata.into_iter().map(|v| MetadataVO::from(v)).collect();
    }
    Vec::new()
}

#[tauri::command]
pub async fn get_metadata_by_id(id: String) -> MetadataVO {
    let mut session = Session::new("./db/main.db");
    session.connect().await;
    let sql = format!("SELECT * FROM metadata WHERE id = {}", id);
    if let Some(metadata) = session.select_one_as::<Metadata>(&sql).await.print_error() {
        return MetadataVO::from(metadata);
    }
    MetadataVO::empty()
}

#[tauri::command]
pub async fn del_metadata(id: i64) -> bool {
    let mut session = Session::new("./db/main.db");
    session.connect().await;
    let sql = format!("UPDATE metadata SET is_del = 1 WHERE id = {}", id);
    session.execute(&sql).await.print_error().is_some()
}
