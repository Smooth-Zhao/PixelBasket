use crate::config::get_db_path;
use crate::data::config::Config;
use crate::util::sqlite::Session;

#[tauri::command]
pub async fn save_config(config: Config) {
    let mut session = Session::new(get_db_path());
    session.connect().await;
    config.save(&session).await;
}

#[tauri::command]
pub async fn update_config(config: Config) {
    let mut session = Session::new(get_db_path());
    session.connect().await;
    config.update(&session).await;
}

#[tauri::command]
pub async fn get_config(key: String) -> Option<Config> {
    let mut session = Session::new(get_db_path());
    session.connect().await;
    Config::get(key, &session).await
}

#[tauri::command]
pub async fn del_config(key: String) {
    let mut session = Session::new(get_db_path());
    session.connect().await;
    Config::delete(key, &session).await;
}