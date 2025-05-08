use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use crate::domain::dto::asset_type::{NewAssetTypeDto, ResAssetTypeDto};
use crate::domain::repositories::asset_type::{AssetTypeError, AssetTypeValidation};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = crate::domain::entities::schema::AssetType)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AssetType {
    pub id: Option<i32>,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::AssetType)]
pub struct NewAssetType {
    pub name: String,
}

impl AssetTypeValidation for NewAssetType {}

impl TryFrom<NewAssetTypeDto> for NewAssetType {
    type Error = AssetTypeError;

    fn try_from(dto: NewAssetTypeDto) -> Result<Self, Self::Error> {
        // Validate the name using the trait implementation
        Self::validate_name(&dto.name)?;
        
        Ok(NewAssetType {
            name: dto.name,
        })
    }
}

impl From<AssetType> for ResAssetTypeDto {
    fn from(model: AssetType) -> Self {
        ResAssetTypeDto {
            id: model.id.unwrap_or(0),
            name: model.name,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl AssetType {
    pub fn new(name: String) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: None,
            name,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn update(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now().to_rfc3339();
    }
} 