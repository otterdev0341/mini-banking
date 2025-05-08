use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::Asset)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Asset {
    pub id: Option<i32>,
    pub name: String,
    pub asset_type_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::Asset)]
pub struct NewAsset {
    pub name: String,
    pub asset_type_id: i32,
} 