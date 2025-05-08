use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::ContactType)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ContactType {
    pub id: Option<i32>,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::ContactType)]
pub struct NewContactType {
    pub name: String,
} 