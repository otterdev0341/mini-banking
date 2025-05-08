use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewContactTypeDto {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResContactTypeDto {
    pub id: i32,
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
} 