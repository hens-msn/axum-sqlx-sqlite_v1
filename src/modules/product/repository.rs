use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use sqlx::Error as SqlxError;
use uuid::Uuid;
use chrono::Utc;
use super::{model::Product, dto::UpdateProductDto};

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create(&self, name: &str, price: f64, stock: i32) -> Result<Product, SqlxError>;
    async fn find_all(&self) -> Result<Vec<Product>, SqlxError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, SqlxError>;
    async fn update(&self, id: &str, dto: UpdateProductDto) -> Result<Product, SqlxError>;
    async fn delete(&self, id: &str) -> Result<u64, SqlxError>;
}

pub struct ProductRepositoryImpl {
    pool: SqlitePool,
}

impl ProductRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn parse_uuid(id: &str) -> Result<Uuid, SqlxError> {
        Uuid::parse_str(id).map_err(|e| SqlxError::Decode(Box::new(e)))
    }
}

#[async_trait]
impl ProductRepository for ProductRepositoryImpl {
    async fn create(&self, name: &str, price: f64, stock: i32) -> Result<Product, SqlxError> {
        let id = Uuid::now_v7();
        let now = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO products (id, name, price, stock, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(price)
        .bind(stock)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(Product {
            id,
            name: name.to_string(),
            price,
            stock,
            created_at: now,
            updated_at: now,
        })
    }

    async fn find_all(&self) -> Result<Vec<Product>, SqlxError> {
        let products = sqlx::query_as::<_, Product>(
            "SELECT id, name, price, stock, created_at, updated_at FROM products"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(products)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, SqlxError> {
        let uuid = Self::parse_uuid(id)?;
        let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = ?")
            .bind(uuid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(product)
    }

    async fn update(&self, id: &str, dto: UpdateProductDto) -> Result<Product, SqlxError> {
        let uuid = Self::parse_uuid(id)?;
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            UPDATE products 
            SET 
                name = COALESCE(?, name),
                price = COALESCE(?, price),
                stock = COALESCE(?, stock),
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(dto.name)
        .bind(dto.price)
        .bind(dto.stock)
        .bind(&now)
        .bind(uuid)
        .execute(&self.pool)
        .await?;

        self.find_by_id(id)
            .await?
            .ok_or(SqlxError::RowNotFound)
    }

    async fn delete(&self, id: &str) -> Result<u64, SqlxError> {
        let uuid = Self::parse_uuid(id)?;
        sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(uuid)
            .execute(&self.pool)
            .await
            .map(|result| result.rows_affected())
    }
}