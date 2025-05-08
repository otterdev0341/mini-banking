use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::CurrentSheet)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CurrentSheet {
    #[diesel(sql_type = Integer)]
    pub id: Option<i32>,
    #[diesel(sql_type = Integer)]
    pub asset_id: i32,
    #[diesel(sql_type = Double)]
    pub balance: f64,
    #[diesel(sql_type = Text)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::CurrentSheet)]
pub struct NewCurrentSheet {
    #[diesel(sql_type = Integer)]
    pub asset_id: i32,
    #[diesel(sql_type = Double)]
    pub balance: f64,
} 