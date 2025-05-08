use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::TransactionRecord)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TransactionRecord {
    #[diesel(sql_type = Integer)]
    pub id: Option<i32>,
    #[diesel(sql_type = Text)]
    pub transaction_type: String,
    #[diesel(sql_type = Double)]
    pub amount: f64,
    #[diesel(sql_type = Integer)]
    pub asset_id: i32,
    #[diesel(sql_type = Integer)]
    pub destination_asset_id: Option<i32>,
    #[diesel(sql_type = Integer)]
    pub expense_id: Option<i32>,
    #[diesel(sql_type = Integer)]
    pub contact_id: Option<i32>,
    #[diesel(sql_type = Text)]
    pub note: Option<String>,
    #[diesel(sql_type = Text)]
    pub created_at: String,
    #[diesel(sql_type = Text)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::TransactionRecord)]
pub struct NewTransactionRecord {
    #[diesel(sql_type = Text)]
    pub transaction_type: String,
    #[diesel(sql_type = Double)]
    pub amount: f64,
    #[diesel(sql_type = Integer)]
    pub asset_id: i32,
    #[diesel(sql_type = Integer)]
    pub destination_asset_id: Option<i32>,
    #[diesel(sql_type = Integer)]
    pub expense_id: Option<i32>,
    #[diesel(sql_type = Integer)]
    pub contact_id: Option<i32>,
    #[diesel(sql_type = Text)]
    pub note: Option<String>,
} 