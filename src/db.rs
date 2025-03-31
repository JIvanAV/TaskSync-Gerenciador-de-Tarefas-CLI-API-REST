use sqlx::{Pool, Sqlite, sqlite::SqlitePool};
use std::env;

pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or("sqlite:tasksync.db".to_string());
    let pool = SqlitePool::connect(&database_url).await?;
    Ok(pool)
}
