use sqlx::{Error, sqlite::SqlitePool};

pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    SqlitePool::connect(&database_url).await
}

pub async fn ping_db(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
