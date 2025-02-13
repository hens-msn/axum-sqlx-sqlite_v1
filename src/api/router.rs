use axum::{Router, routing::get};
use sqlx::SqlitePool;
use crate::modules::product::route::product_router;

pub fn create_router(db_pool: SqlitePool) -> Router {
    // Buat sub-router untuk API
    let api_router = Router::new()
        .nest("/products", product_router(db_pool))  // 🛠️
        // .nest("/users", user_router)  // Contoh nanti kalo ada modul user
        ;

    Router::new()
        .nest("/api", api_router)  // 🌟 Semua route API di-group di sini
        .route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "🫀 Hidup dan sehat!"
}
