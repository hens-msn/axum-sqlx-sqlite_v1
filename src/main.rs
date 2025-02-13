use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("🚀 Server berjalan di {}", std::env::var("SERVER_HOST")?);
    
    // Database pool
    let db_pool = axum_sqlx_sqlite_v1::core::db::connect().await?;
    
    // Router
    let app = axum_sqlx_sqlite_v1::api::router::create_router(db_pool);
    
    // Run server
    let addr = std::env::var("SERVER_HOST")?.parse::<std::net::SocketAddr>()?;
        
    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app
    ).await?;

    Ok(())
}
