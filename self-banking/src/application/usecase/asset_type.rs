use async_trait::async_trait;
use crate::domain::model::asset_type::{AssetType, NewAssetType};
use crate::domain::repositories::asset_type::{AssetTypeRepository, AssetTypeError};
use crate::domain::dto::asset_type::{NewAssetTypeDto, ResAssetTypeDto};

pub struct AssetTypeUseCase<R: AssetTypeRepository> {
    repository: R,
}

impl<R: AssetTypeRepository> AssetTypeUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_asset_type(&self, dto: NewAssetTypeDto) -> Result<ResAssetTypeDto, AssetTypeError> {
        // Convert DTO to domain model
        let new_asset_type = NewAssetType::try_from(dto)?;
        
        // Create asset type using repository
        let asset_type = self.repository.create(new_asset_type).await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => AssetTypeError::NameAlreadyExists("".to_string()),
                _ => AssetTypeError::DatabaseError(e),
            })?;
        
        // Convert domain model to response DTO
        Ok(ResAssetTypeDto::from(asset_type))
    }

    pub async fn get_all_asset_types(&self) -> Result<Vec<ResAssetTypeDto>, AssetTypeError> {
        let asset_types = self.repository.get_all().await
            .map_err(AssetTypeError::DatabaseError)?;
        
        Ok(asset_types.into_iter()
            .map(ResAssetTypeDto::from)
            .collect())
    }

    pub async fn get_asset_type_by_id(&self, id: i32) -> Result<Option<ResAssetTypeDto>, AssetTypeError> {
        let asset_type = self.repository.get_by_id(id).await
            .map_err(AssetTypeError::DatabaseError)?;
        
        Ok(asset_type.map(ResAssetTypeDto::from))
    }

    pub async fn update_asset_type(&self, id: i32, dto: NewAssetTypeDto) -> Result<ResAssetTypeDto, AssetTypeError> {
        // Convert DTO to domain model
        let new_asset_type = NewAssetType::try_from(dto)?;
        
        // Update asset type using repository
        let asset_type = self.repository.update(id, new_asset_type).await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => AssetTypeError::NotFound(id),
                _ => AssetTypeError::DatabaseError(e),
            })?;
        
        // Convert domain model to response DTO
        Ok(ResAssetTypeDto::from(asset_type))
    }

    pub async fn delete_asset_type(&self, id: i32) -> Result<bool, AssetTypeError> {
        // Check if asset type exists
        let exists = self.repository.exists(id).await
            .map_err(AssetTypeError::DatabaseError)?;
        
        if !exists {
            return Err(AssetTypeError::NotFound(id));
        }
        
        // Delete asset type
        self.repository.delete(id).await
            .map_err(AssetTypeError::DatabaseError)
    }

    pub async fn find_asset_types_by_name(&self, name: &str) -> Result<Vec<ResAssetTypeDto>, AssetTypeError> {
        let asset_types = self.repository.find_by_name(name).await
            .map_err(AssetTypeError::DatabaseError)?;
        
        Ok(asset_types.into_iter()
            .map(ResAssetTypeDto::from)
            .collect())
    }

    pub async fn count_asset_types(&self) -> Result<i64, AssetTypeError> {
        self.repository.count().await
            .map_err(AssetTypeError::DatabaseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        AssetTypeRepository {}
        #[async_trait]
        impl AssetTypeRepository for AssetTypeRepository {
            async fn create(&self, new_asset_type: NewAssetType) -> Result<AssetType, diesel::result::Error>;
            async fn get_by_id(&self, id: i32) -> Result<Option<AssetType>, diesel::result::Error>;
            async fn get_all(&self) -> Result<Vec<AssetType>, diesel::result::Error>;
            async fn update(&self, id: i32, asset_type: NewAssetType) -> Result<AssetType, diesel::result::Error>;
            async fn delete(&self, id: i32) -> Result<bool, diesel::result::Error>;
            async fn find_by_name(&self, name: &str) -> Result<Vec<AssetType>, diesel::result::Error>;
            async fn exists(&self, id: i32) -> Result<bool, diesel::result::Error>;
            async fn count(&self) -> Result<i64, diesel::result::Error>;
        }
    }

    #[tokio::test]
    async fn test_create_asset_type() {
        let mut mock_repo = MockAssetTypeRepository::new();
        let usecase = AssetTypeUseCase::new(mock_repo);
        
        let dto = NewAssetTypeDto {
            name: "Test Asset Type".to_string(),
        };
        
        let result = usecase.create_asset_type(dto).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_all_asset_types() {
        let mut mock_repo = MockAssetTypeRepository::new();
        let usecase = AssetTypeUseCase::new(mock_repo);
        
        let result = usecase.get_all_asset_types().await;
        assert!(result.is_ok());
    }
} 