use super::{repository::ProductRepository, model::Product, dto::UpdateProductDto};
use super::repository::ProductRepositoryImpl;
use std::sync::Arc;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct ProductService {
    repo: Arc<dyn ProductRepository>,
}

impl ProductService {
    pub fn new(repo: Arc<dyn ProductRepository>) -> Self {
        Self { repo }
    }

    pub fn with_db_pool(pool: SqlitePool) -> Self {
        let repo = Arc::new(ProductRepositoryImpl::new(pool));
        Self::new(repo)
    }

    pub async fn create_product(&self, name: &str, price: f64, stock: i32) -> Result<Product, sqlx::Error> {
        self.repo.create(name, price, stock).await
    }

    pub async fn get_all_products(&self) -> Result<Vec<Product>, sqlx::Error> {
        self.repo.find_all().await
    }

    pub async fn get_product(&self, id: &str) -> Result<Option<Product>, sqlx::Error> {
        self.repo.find_by_id(id).await
    }

    pub async fn update_product(&self, id: &str, dto: UpdateProductDto) -> Result<Product, sqlx::Error> {
        self.repo.update(id, dto).await
    }

    pub async fn delete_product(&self, id: &str) -> Result<u64, sqlx::Error> {
        self.repo.delete(id).await
    }
}
