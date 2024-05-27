use std::ops::Add;
use std::path::MAIN_SEPARATOR_STR;

use crate::config::get_db_path;
use crate::data::metadata::{Metadata, MetadataVO};
use crate::util::error::ErrorHandle;
use crate::util::sqlite::Session;

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
        "SELECT * FROM metadata WHERE file_path {} '{}{}'",
        if like { "LIKE" } else { "=" },
        path.add(MAIN_SEPARATOR_STR),
        if like { "%" } else { "" },
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