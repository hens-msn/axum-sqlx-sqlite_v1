use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProductDto {
    #[validate(
        length(min = 3, max = 100, message = "Nama produk harus 3-100 karakter 📏")
    )]
    pub name: String,
    
    #[validate(
        range(min = 0.0, message = "Harga tidak boleh minus 🔻")
    )]
    pub price: f64,
    
    #[validate(
        range(min = 0, message = "Stok tidak boleh minus 🔻")
    )]
    pub stock: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_update_dto"))]
pub struct UpdateProductDto {
    #[validate(
        length(min = 3, max = 100, message = "Nama produk harus 3-100 karakter 📏")
    )]
    pub name: Option<String>,
    
    #[validate(
        range(min = 0.0, message = "Harga tidak boleh minus 🔻")
    )]
    pub price: Option<f64>,
    
    #[validate(
        range(min = 0, message = "Stok tidak boleh minus 🔻")
    )]
    pub stock: Option<i32>,
}

fn validate_update_dto(dto: &UpdateProductDto) -> Result<(), ValidationError> {
    // Cek minimal ada 1 field yg diisi
    if dto.name.is_none() && dto.price.is_none() && dto.stock.is_none() {
        return Err(ValidationError::new("Harus isi minimal satu field 🚦"));
    }
    
    // Validasi conditional
    if dto.name.is_none() && dto.stock.is_some() {
        if dto.price.is_none() {
            return Err(ValidationError::new("Kalo update stok, harga harus diisi 💸"));
        }
    }
    
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success {
        success: bool,
        message: String,
        data: T,
    },
    Error {
        success: bool,
        message: String,
        error: String,
    },
    ValidationError {
        success: bool,
        message: String,
        errors: Vec<String>,
    },
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self::Success {
            success: true,
            message: message.to_string(),
            data,
        }
    }

    pub fn error(message: &str, error: &str) -> Self {
        Self::Error {
            success: false,
            message: message.to_string(),
            error: error.to_string(),
        }
    }
    
    pub fn validation_error(errors: Vec<String>) -> Self {
        Self::ValidationError {
            success: false,
            message: "Validation error".to_string(),
            errors,
        }
    }
}
