use sqlx::{SqlitePool, migrate::MigrateDatabase, Sqlite};
use anyhow::Result;
use std::env;

pub async fn init_db() -> Result<SqlitePool> {
    // Get database path from environment variable or use default
    let database_path = env::var("DATABASE_PATH").unwrap_or_else(|_| "./config.db".to_string());
    let database_url = format!("sqlite:{}", database_path);
    
    println!("📂 Using database: {}", database_path);
    
    if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        println!("🔨 Creating new database at: {}", database_path);
        Sqlite::create_database(&database_url).await?;
    }
    
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Run migrations
    println!("🔄 Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("✅ Database migrations completed");
    
    Ok(pool)
}