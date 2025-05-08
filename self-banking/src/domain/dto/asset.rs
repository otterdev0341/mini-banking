use serde::{Serialize, Deserialize};
use validator::Validate;
use super::asset_type::ResAssetTypeDto;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewAssetDto {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub asset_type_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResAssetDto {
    pub id: i32,
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub asset_type: ResAssetTypeDto,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct ResAssetWithBalanceDto {
    pub id: i32,
    pub name: String,
    pub asset_type: ResAssetTypeDto,
    pub balance: f64,
    pub created_at: String,
    pub updated_at: String,
} 