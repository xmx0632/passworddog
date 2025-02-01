use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::env;
use std::str::FromStr;
use log::info;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or("sqlite:data/passworddog.db".to_string());
    
    // 创建连接选项
    let options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .foreign_keys(true);
    
    // 创建连接池
    let pool = SqlitePool::connect_with(options).await?;
    
    // 运行初始化SQL
    info!("Initializing database...");
    let sql = include_str!("../../migrations/20250201_init.sql");
    sqlx::query(sql).execute(&pool).await?;
    
    Ok(pool)
}
