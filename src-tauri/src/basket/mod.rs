use std::vec;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::channel;

use crate::config::get_db_path;
use crate::db::entity::basket::{Basket, BasketData, BasketVO};
use crate::db::entity::folder::{Folder, FolderVO};
use crate::db::entity::metadata::{Metadata, MetadataVO};
use crate::db::sqlite::Session;
use crate::file::image_scanner::ImageScanner;
use crate::file::model_scanner::ModelScanner;
use crate::file::psd_scanner::PsdScanner;
use crate::file::raw_scanner::RawScanner;
use crate::file::scan::{ScanJob, ScanMsg};
use crate::file::video_scanner::VideoScanner;
use crate::util::error::ErrorHandle;

#[tauri::command]
pub fn create_basket(basket: BasketData) -> &'static str {
    let (tx, rx) = channel::<ScanMsg>(16);
    let mut scan = ScanJob::new(tx);
    scan.add_scanners(vec![
        ImageScanner::wrap(),
        ModelScanner::wrap(),
        VideoScanner::wrap(),
        RawScanner::wrap(),
        PsdScanner::wrap(),
    ]);
    scan.monitor_async(rx);
    scan.run_async(basket);
    "OK"
}

#[tauri::command]
pub fn run_task() -> &'static str {
    let (tx, rx) = channel::<ScanMsg>(16);
    let mut scan = ScanJob::new(tx);
    scan.add_scanners(vec![
        ImageScanner::wrap(),
        ModelScanner::wrap(),
        VideoScanner::wrap(),
        RawScanner::wrap(),
        PsdScanner::wrap(),
    ]);
    scan.monitor_async(rx);
    scan.run_task_async();
    "OK"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    size: usize,
    current: usize,
}

#[tauri::command]
pub async fn get_metadata() -> Vec<MetadataVO> {
    let mut session = Session::new(get_db_path());
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
    let mut session = Session::new(get_db_path());
    session.connect().await;
    let sql = format!("SELECT * FROM metadata WHERE id = {}", id);
    if let Some(metadata) = session.select_one_as::<Metadata>(&sql).await.print_error() {
        return MetadataVO::from(metadata);
    }
    MetadataVO::empty()
}

#[tauri::command]
pub async fn get_metadata_like_path(path: String, like: bool) -> Vec<MetadataVO> {
    let mut session = Session::new(get_db_path());
    session.connect().await;
    let sql = format!(
        "SELECT * FROM metadata WHERE file_path {} '{}%'",
        if like { "LIKE" } else { "=" },
        path
    );
    if let Some(metadata) = session.select_as::<Metadata>(&sql).await.print_error() {
        return metadata.into_iter().map(|v| MetadataVO::from(v)).collect();
    }
    Vec::new()
}

#[tauri::command]
pub async fn del_metadata(id: String) -> bool {
    let mut session = Session::new(get_db_path());
    session.connect().await;
    session
        .execute(&format!("UPDATE metadata SET is_del = 1 WHERE id = {}", id))
        .await
        .print_error()
        .is_some()
}

#[tauri::command]
pub async fn get_basket() -> Vec<BasketVO> {
    let mut session = Session::new(get_db_path());
    session.connect().await;
    if let Some(basket) = session
        .select_as::<Basket>("SELECT * FROM basket")
        .await
        .print_error()
    {
        return basket.into_iter().map(|v| BasketVO::from(v)).collect();
    }
    Vec::new()
}

#[tauri::command]
pub async fn del_basket(id: String) -> bool {
    let mut session = Session::new(get_db_path());
    session.connect().await;
    if session
        .execute(&format!("DELETE FROM basket WHERE id = {id}"))
        .await
        .print_error()
        .is_some()
    {
        return session
            .execute(&format!("DELETE FROM basket_folder WHERE basket_id = {id}"))
            .await
            .print_error()
            .is_some();
    }
    false
}

#[tauri::command]
pub async fn get_folder(id: String) -> Vec<FolderVO> {
    let mut session = Session::new(get_db_path());
    session.connect().await;
    if let Some(folder) = session
        .select_as::<Folder>(&format!(
            r#"
            WITH RECURSIVE descendants AS (SELECT *
                                           FROM folder
                                           WHERE id IN (SELECT bf.folder_id
                                                        FROM basket b
                                                                 LEFT JOIN basket_folder bf ON bf.basket_id = b.id
                                                        WHERE b.id = {id})
                                           UNION ALL
                                           SELECT child.*
                                           FROM folder AS child
                                                    JOIN descendants ON child.pid = descendants.id)
            SELECT *
            FROM descendants
            GROUP BY id
            ORDER BY path;
            "#
        ))
        .await
        .print_error()
    {
        return folder.into_iter().map(|v| FolderVO::from(v)).collect();
    }
    Vec::new()
}
