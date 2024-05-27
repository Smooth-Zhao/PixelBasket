use tokio::sync::mpsc::channel;

use crate::config::get_db_path;
use crate::data::basket::{Basket, BasketData, BasketVO};
use crate::scanner::image_scanner::ImageScanner;
use crate::scanner::model_scanner::ModelScanner;
use crate::scanner::psd_scanner::PsdScanner;
use crate::scanner::raw_scanner::RawScanner;
use crate::scanner::scan::{ScanJob, ScanMsg};
use crate::scanner::video_scanner::VideoScanner;
use crate::util::error::ErrorHandle;
use crate::util::sqlite::Session;

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
        let mut result = session
            .execute(&format!(
                r#"
                DELETE
                FROM metadata
                WHERE id IN (SELECT m.id
                             FROM (SELECT COUNT(bf.folder_id) AS count, f.path
                                   FROM basket_folder bf
                                            LEFT JOIN basket_folder ibf ON ibf.folder_id = bf.folder_id
                                            LEFT JOIN folder f ON f.id = bf.folder_id
                                   WHERE bf.basket_id = {id}
                                   GROUP BY ibf.folder_id
                                   HAVING count = 1) t
                                      LEFT JOIN metadata m ON m.file_path LIKE concat(t.path, '%'))
                "#
            ))
            .await
            .print_error()
            .is_some();
        result &= session
            .execute(&format!(
                r#"
                DELETE
                FROM folder
                WHERE id IN (SELECT id
                             FROM (WITH RECURSIVE descendants AS (SELECT *
                                                                  FROM folder
                                                                  WHERE id IN (SELECT bf.folder_id
                                                                               FROM basket_folder bf
                                                                               WHERE bf.basket_id = {id})
                                                                  UNION ALL
                                                                  SELECT child.*
                                                                  FROM folder AS child
                                                                           JOIN descendants ON child.pid = descendants.id)
                                   SELECT *
                                   FROM descendants
                                   GROUP BY id
                                   ORDER BY path) f1
                             WHERE f1.id NOT IN (SELECT id
                                                 FROM (WITH RECURSIVE descendants AS (SELECT *
                                                                                      FROM folder
                                                                                      WHERE id IN (SELECT bf.folder_id
                                                                                                   FROM basket_folder bf
                                                                                                   WHERE bf.basket_id != {id})
                                                                                      UNION ALL
                                                                                      SELECT child.*
                                                                                      FROM folder AS child
                                                                                               JOIN descendants ON child.pid = descendants.id)
                                                       SELECT *
                                                       FROM descendants
                                                       GROUP BY id
                                                       ORDER BY path)))
                "#
            ))
            .await
            .print_error()
            .is_some();
        result &= session
            .execute(&format!("DELETE FROM basket_folder WHERE basket_id = {id}"))
            .await
            .print_error()
            .is_some();
        return result;
    }
    false
}
