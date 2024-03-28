use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::file::get_directory_tree;

#[derive(Debug,Serialize,Deserialize)]
pub struct Basket{
    name: String,
    directories: Vec<String>
}

#[tauri::command]
pub async fn create_basket(basket:Basket) -> &'static str {
    // println!("{:?}",basket);
    for x in basket.directories.iter() {
        let string = x.clone();
        tokio::spawn(async move {
            get_directory_tree(Path::new(string.as_str()))
        });
    }
    "OK"
}
