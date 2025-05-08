use serde::{Serialize, Deserialize};
use validator::Validate;
use super::{asset::ResAssetDto, expense::ResExpenseDto, contact::ResContactDto};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewTransactionDto {
    #[validate(length(min = 1, max = 50))]
    pub transaction_type: String,
    #[validate(range(min = 0.01))]
    pub amount: f64,
    pub asset_id: i32,
    pub destination_asset_id: Option<i32>,
    pub expense_id: Option<i32>,
    pub contact_id: Option<i32>,
    #[validate(length(max = 500))]
    pub note: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ResTransactionDto {
    pub id: i32,
    pub transaction_type: String,
    pub amount: f64,
    pub asset: ResAssetDto,
    pub destination_asset: Option<ResAssetDto>,
    pub expense: Option<ResExpenseDto>,
    pub contact: Option<ResContactDto>,
    pub note: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// Special request DTOs for specific operations
#[derive(Debug, Deserialize, Validate)]
pub struct NewTransferDto {
    pub from_asset_id: i32,
    pub to_asset_id: i32,
    #[validate(range(min = 0.01))]
    pub amount: f64,
    #[validate(length(max = 500))]
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewExpenseTransactionDto {
    pub asset_id: i32,
    pub expense_type_id: i32,
    #[validate(length(min = 1, max = 200))]
    pub description: String,
    #[validate(range(min = 0.01))]
    pub amount: f64,
    pub contact_id: Option<i32>,
    #[validate(length(max = 500))]
    pub note: Option<String>,
} 