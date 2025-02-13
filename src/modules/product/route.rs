use axum::{
    routing::{get, post, put, delete},
    Router, extract::{State, Path}, Json,
    response::IntoResponse,
    http::StatusCode
};
use anyhow::anyhow;
use super::service::ProductService;
use super::model::Product;
use super::dto::{CreateProductDto, ProductResponse, UpdateProductDto, ApiResponse};
use validator::Validate;
use sqlx::SqlitePool;

#[derive(Debug)]
struct RouteError(anyhow::Error);

impl IntoResponse for RouteError {
    fn into_response(self) -> axum::response::Response {
        let status = match self.0.downcast_ref::<sqlx::Error>() {
            Some(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        (status, self.0.to_string()).into_response()
    }
}

impl From<sqlx::Error> for RouteError {
    fn from(err: sqlx::Error) -> Self {
        RouteError(anyhow!(err))
    }
}

pub fn product_router(db_pool: SqlitePool) -> Router {
    let service = ProductService::with_db_pool(db_pool);
    
    Router::new()
        .route("/", post(create_product))
        .route("/", get(get_all_products))
        .route("/{id}", get(get_product))
        .route("/{id}", put(update_product))
        .route("/{id}", delete(delete_product))
        .with_state(service)
}

async fn create_product(
    State(service): State<ProductService>,
    Json(payload): Json<CreateProductDto>,
) -> Result<Json<ApiResponse<ProductResponse>>, RouteError> {
    // Validasi payload
    payload.validate()
        .map_err(|e| RouteError(anyhow!(e)))?;  // 🛑 Validasi gagal
    
    let product = service
        .create_product(&payload.name, payload.price, payload.stock)
        .await?;
        
    Ok(Json(ApiResponse::success(
        map_product_to_response(product),
        "Produk berhasil dibuat 🎉"
    )))
}

async fn get_all_products(
    State(service): State<ProductService>,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, RouteError> {
    let products = service.get_all_products().await?;
    Ok(Json(ApiResponse::success(
        products.into_iter().map(map_product_to_response).collect(),
        "Daftar produk berhasil diambil 📦"
    )))
}

async fn get_product(
    State(service): State<ProductService>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ProductResponse>>, RouteError> {
    let product = service.get_product(&id)
        .await?
        .ok_or(RouteError(anyhow!("Produk tidak ditemukan 🕵️♂️")))?;
        
    Ok(Json(ApiResponse::success(
        map_product_to_response(product),
        "Detail produk berhasil diambil 🔍"
    )))
}

async fn update_product(
    State(service): State<ProductService>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProductDto>,
) -> Result<Json<ApiResponse<ProductResponse>>, RouteError> {
    // Validasi payload update
    payload.validate()
        .map_err(|e| RouteError(anyhow!(e)))?;  // 🛑 Validasi gagal
    
    let product = service.update_product(&id, payload).await?;
    Ok(Json(ApiResponse::success(
        map_product_to_response(product), 
        "Produk berhasil diupdate ✨"
    )))
}

async fn delete_product(
    State(service): State<ProductService>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<u64>>, RouteError> {
    let rows_affected = service.delete_product(&id).await?;
    Ok(Json(ApiResponse::success(
        rows_affected,
        "Produk berhasil dihapus 🗑️"
    )))
}

fn map_product_to_response(product: Product) -> ProductResponse {
    ProductResponse {
        id: product.id,
        name: product.name,
        price: product.price,
        stock: product.stock,
        created_at: product.created_at.to_rfc3339(),
        updated_at: product.updated_at.to_rfc3339(),
    }
}
