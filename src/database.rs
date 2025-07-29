use sqlx::{SqlitePool, migrate::MigrateDatabase, Sqlite};
use anyhow::Result;

pub async fn init_db() -> Result<SqlitePool> {
    let database_url = "sqlite:./config.db";
    
    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        Sqlite::create_database(database_url).await?;
    }
    
    let pool = SqlitePool::connect(database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}