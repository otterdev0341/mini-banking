use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::Expense)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Expense {
    pub id: Option<i32>,
    pub description: String,
    pub expense_type_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::domain::entities::schema::Expense)]
pub struct NewExpense {
    pub description: String,
    pub expense_type_id: i32,
} 