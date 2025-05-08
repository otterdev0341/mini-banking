use async_trait::async_trait;
use diesel::result::Error;
use crate::domain::model::asset_type::{AssetType, NewAssetType};

#[async_trait]
pub trait AssetTypeRepository: Send + Sync {
    /// Create a new asset type
    async fn create(&self, new_asset_type: NewAssetType) -> Result<AssetType, Error>;

    /// Get an asset type by ID
    async fn get_by_id(&self, id: i32) -> Result<Option<AssetType>, Error>;

    /// Get all asset types
    async fn get_all(&self) -> Result<Vec<AssetType>, Error>;

    /// Update an asset type
    async fn update(&self, id: i32, asset_type: NewAssetType) -> Result<AssetType, Error>;

    /// Delete an asset type by ID
    async fn delete(&self, id: i32) -> Result<bool, Error>;

    /// Find asset types by name (partial match)
    async fn find_by_name(&self, name: &str) -> Result<Vec<AssetType>, Error>;

    /// Check if an asset type exists by ID
    async fn exists(&self, id: i32) -> Result<bool, Error>;

    /// Count total number of asset types
    async fn count(&self) -> Result<i64, Error>;
}

/// Error type for AssetType operations
#[derive(Debug, thiserror::Error)]
pub enum AssetTypeError {
    #[error("Asset type not found with ID: {0}")]
    NotFound(i32),

    #[error("Database error: {0}")]
    DatabaseError(#[from] Error),

    #[error("Asset type with name '{0}' already exists")]
    NameAlreadyExists(String),

    #[error("Invalid asset type data: {0}")]
    ValidationError(String),
}

/// Implementation helper for common validation logic
pub trait AssetTypeValidation {
    fn validate_name(name: &str) -> Result<(), AssetTypeError> {
        if name.trim().is_empty() {
            return Err(AssetTypeError::ValidationError("Name cannot be empty".to_string()));
        }
        if name.len() > 50 {
            return Err(AssetTypeError::ValidationError("Name is too long (maximum 50 characters)".to_string()));
        }
        Ok(())
    }
} 