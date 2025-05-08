use serde::{Serialize, Deserialize};
use validator::Validate;
use super::expense_type::ResExpenseTypeDto;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewExpenseDto {
    #[validate(length(min = 1, max = 200))]
    pub description: String,
    pub expense_type_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResExpenseDto {
    pub id: i32,
    #[validate(length(min = 1, max = 200))]
    pub description: String,
    pub expense_type: ResExpenseTypeDto,
    pub created_at: String,
    pub updated_at: String,
} 