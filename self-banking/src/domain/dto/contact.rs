use serde::{Serialize, Deserialize};
use validator::Validate;
use super::contact_type::ResContactTypeDto;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewContactDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub business_name: String,
    #[validate(length(min = 10, max = 20))]
    pub phone: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub contact_type_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResContactDto {
    pub id: i32,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub business_name: String,
    #[validate(length(min = 10, max = 20))]
    pub phone: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub contact_type: ResContactTypeDto,
    pub created_at: String,
    pub updated_at: String,
} 