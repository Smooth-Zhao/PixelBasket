use std::env;
use std::error::Error;
use sqlx::{migrate::MigrateDatabase,Sqlite};
pub async fn query_from_sqlite() -> Result<(), Box<dyn Error>> {
    let db_url = env::var("DATABASE_URL").expect("PORT environment variable is not set");
    if !Sqlite::database_exists(db_url.as_str()).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url.as_str()).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    Ok(())
}
