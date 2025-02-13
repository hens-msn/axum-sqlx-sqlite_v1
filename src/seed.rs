use sqlx::{SqlitePool, Error};
use serde::Deserialize;
use include_dir::{include_dir, Dir};

static DATA_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/data");

#[derive(Debug, Deserialize)]
struct SeedProduct {
    name: String,
    price: f64,
    stock: i32,
}

pub async fn seed_products(pool: &SqlitePool) -> Result<(), Error> {
    // Baca file JSON
    let json_file = DATA_DIR.get_file("products.json")
        .ok_or_else(|| Error::Configuration("File products.json tidak ditemukan 😱".into()))?;
    
    let products: Vec<SeedProduct> = serde_json::from_slice(json_file.contents())
        .map_err(|e| Error::Configuration(format!("Gagal parse JSON: {}", e).into()))?;

    // Bersihin data lama (SQLite ga support TRUNCATE)
    sqlx::query("DELETE FROM products")
        .execute(pool)
        .await?;

    // Insert data baru
    for product in products {
        // Panggil service create product
        let id = uuid::Uuid::now_v7();
        let now = chrono::Utc::now().to_rfc3339();

        let _ = sqlx::query(
            r#"
            INSERT INTO products (id, name, price, stock, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(product.name)
        .bind(product.price)
        .bind(product.stock)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    println!("✅ Seeder dari JSON berhasil! 🌱");
    Ok(())
} 