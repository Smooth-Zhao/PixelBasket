use sqlx_sqlite::SqlitePoolOptions;

pub async fn query_from_sqlite() -> Result<(), Box<dyn Error>> {
    // 设置连接选项并创建数据库连接池
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:./test.db")
        .await?;

    // 执行查询
    let row = sqlx::query!("SELECT * FROM your_table")
        .fetch_one(&pool)
        .await?;

    // 处理查询结果
    let column_value: String = row.column_name;
    println!("Column value: {}", column_value);

    Ok(())
}
