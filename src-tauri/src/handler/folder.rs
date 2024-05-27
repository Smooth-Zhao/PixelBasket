use crate::config::get_db_path;
use crate::data::folder::{Folder, FolderVO};
use crate::util::error::ErrorHandle;
use crate::util::sqlite::Session;

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