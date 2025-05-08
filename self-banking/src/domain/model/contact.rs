use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::Contact)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Contact {
    pub id: Option<i32>,
    pub name: String,
    pub business_name: String,
    pub phone: String,
    pub description: Option<String>,
    pub contact_type_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::Contact)]
pub struct NewContact {
    pub name: String,
    pub business_name: String,
    pub phone: String,
    pub description: Option<String>,
    pub contact_type_id: i32,
} 