use dotenv::dotenv;

use pixel_basket::basket;
use pixel_basket::util::error::ErrorHandle;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            basket::create_basket,
            basket::get_metadata,
            basket::del_metadata,
            basket::get_metadata_by_id,
            basket::get_basket,
            basket::del_basket,
            basket::get_folder
        ])
        .run(tauri::generate_context!())
        .print_error();
}
