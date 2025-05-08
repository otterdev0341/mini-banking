use serde::{Serialize, Deserialize};
use validator::Validate;
use super::asset::ResAssetDto;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewCurrentSheetDto {
    pub asset_id: i32,
    pub balance: f64,
}

#[derive(Debug, Serialize)]
pub struct ResCurrentSheetDto {
    pub id: i32,
    pub asset: ResAssetDto,
    pub balance: f64,
    pub updated_at: String,
} 