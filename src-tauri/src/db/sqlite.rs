use sqlx::sqlite::{SqliteConnectOptions, SqliteQueryResult, SqliteRow};
use sqlx::{query, query_as, FromRow, Pool, Sqlite, SqlitePool};

/// 数据库会话
pub struct Session {
    url: String,
    pool: Option<Pool<Sqlite>>,
}

impl Session {
    /// 使用url创建数据库会话
    ///
    /// ```rust,no_run
    /// # use pixel_basket::db::sqlite::Session;
    /// # async fn example() {
    /// let session = Session::new("test.db");
    /// # }
    /// ```
    pub fn new(url: &str) -> Session {
        Session {
            url: url.to_string(),
            pool: None,
        }
    }

    /// 获取连接池
    ///
    /// ```rust,no_run
    /// # use pixel_basket::db::sqlite::Session;
    /// # async fn example() {
    /// let mut session = Session::new("test.db");
    /// // 先建立连接
    /// session.connect().await;
    ///
    /// if let Ok(pool) = session.get_pool() {
    ///     // 使用pool
    /// }
    /// # }
    /// ```
    pub fn get_pool(&self) -> Result<Pool<Sqlite>, sqlx::Error> {
        if let Some(pool) = &self.pool {
            return Ok(pool.clone());
        }
        Err(sqlx::Error::PoolClosed)
    }

    /// 建立连接
    ///
    /// ```rust,no_run
    /// # use pixel_basket::db::sqlite::Session;
    /// # async fn example() {
    /// let mut session = Session::new("test.db");
    /// session.connect().await;
    /// # }
    /// ```
    pub async fn connect(&mut self) {
        let options = SqliteConnectOptions::new().filename(&self.url);
        if let Ok(pool) = SqlitePool::connect_with(options).await {
            self.pool = Some(pool);
        }
    }

    /// 执行语句
    ///
    /// ```rust,no_run
    /// # use pixel_basket::db::sqlite::Session;
    /// # async fn example() {
    /// let mut session = Session::new("test.db");
    /// // 先建立连接
    /// session.connect().await;
    ///
    /// session.execute("SELECT * FROM example").await.expect("");
    /// # }
    /// ```
    pub async fn execute(&self, sql: &str) -> Result<SqliteQueryResult, sqlx::Error> {
        if let Some(pool) = &self.pool {
            return query(sql).execute(pool).await;
        }
        Err(sqlx::Error::PoolClosed)
    }

    /// 查询语句
    ///
    /// ```rust,no_run
    /// use sqlx::sqlite::SqliteRow;
    ///
    /// # use pixel_basket::db::sqlite::Session;
    /// # async fn example() {
    /// let mut session = Session::new("test.db");
    /// // 先建立连接
    /// session.connect().await;
    ///
    /// let result: Vec<SqliteRow> = session.select("SELECT * FROM example").await.expect("");
    /// # }
    pub async fn select(&self, sql: &str) -> Result<Vec<SqliteRow>, sqlx::Error> {
        if let Some(pool) = &self.pool {
            return query(sql).fetch_all(pool).await;
        }
        Err(sqlx::Error::PoolClosed)
    }

    /// 查询语句
    ///
    /// ```rust,no_run
    /// use sqlx::sqlite::SqliteRow;   
    ///
    /// #[derive(sqlx::FromRow)]
    /// struct User {
    ///    // ...
    /// }
    ///
    /// # use pixel_basket::db::sqlite::Session;
    /// # async fn example() {
    /// let mut session = Session::new("test.db");
    /// // 先建立连接
    /// session.connect().await;
    ///
    /// let result: Vec<User> = session.select_as::<User>("SELECT * FROM example").await.expect("");
    /// # }
    pub async fn select_as<T: for<'r> FromRow<'r, SqliteRow> + Send + Unpin>(
        &self,
        sql: &str,
    ) -> Result<Vec<T>, sqlx::Error> {
        if let Some(pool) = &self.pool {
            return query_as::<_, T>(sql).fetch_all(pool).await;
        }
        Err(sqlx::Error::PoolClosed)
    }

    /// 查询条数
    ///
    /// ```rust,no_run
    /// use sqlx::sqlite::SqliteRow;
    ///
    /// # use pixel_basket::db::sqlite::{Count, Session};
    /// # async fn example() {
    /// let mut session = Session::new("test.db");
    /// // 先建立连接
    /// session.connect().await;
    ///
    /// let result: Count = session.count("SELECT COUNT(*) AS count FROM example").await.expect("");
    /// # }
    pub async fn count(&self, sql: &str) -> Result<Count, sqlx::Error> {
        if let Some(pool) = &self.pool {
            return query_as::<_, Count>(sql).fetch_one(pool).await;
        }
        Err(sqlx::Error::PoolClosed)
    }
}

#[cfg(test)]
mod tests {
    use crate::db::sqlite::Session;
    use sqlx::query;

    #[derive(Debug, sqlx::FromRow)]
    struct User {
        id: i32,
        name: String,
    }

    #[tokio::test]
    async fn test_get_pool() {
        let mut session = Session::new("./db/main.db");
        session.connect().await;
        let pool = session.get_pool();
        println!("{:?}", pool)
    }

    #[tokio::test]
    async fn test_connect() {
        let mut session = Session::new("./db/main.db");
        session.connect().await;
    }

    #[tokio::test]
    async fn test_execute() {
        let mut session = Session::new("./db/main.db");
        session.connect().await;
        session.execute("DROP TABLE users").await.expect("err");
        session
            .execute(
                "CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            )
            .await
            .expect("err");
    }

    #[tokio::test]
    async fn test_insert() {
        let mut session = Session::new("./db/main.db");
        session.connect().await;
        let user = User {
            id: 10,
            name: "test1".to_string(),
        };
        query("INSERT INTO users (id, name) VALUES (?, ?)")
            .bind(user.id)
            .bind(user.name)
            .execute(&session.get_pool().expect("err"))
            .await
            .expect("err");
    }

    #[tokio::test]
    async fn test_query() {
        let mut session = Session::new("./db/main.db");
        session.connect().await;
        session
            .execute("INSERT INTO users (name) VALUES ('test')")
            .await
            .expect("err");
        let result = session.select("SELECT * FROM users").await.expect("err");
        println!("len:{}", result.len());
        let result = session
            .select_as::<User>("SELECT * FROM users")
            .await
            .expect("err");
        println!("users:{:?}", result);
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Count {
    pub count: i64,
}
