use async_trait::async_trait;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::domain::model::asset_type::{AssetType, NewAssetType};
use crate::domain::repositories::asset_type::{AssetTypeRepository, AssetTypeError};
use crate::domain::entities::schema::AssetType as AssetTypeTable;
use crate::infrastructure::sqlite::sqlite_connection::{SqliteConnection, ConnectionError};

#[derive(Debug, Clone)]
pub struct AssetTypeRepositoryImpl {
    connection: SqliteConnection,
}

impl AssetTypeRepositoryImpl {
    pub fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl AssetTypeRepository for AssetTypeRepositoryImpl {
    async fn create(&self, new_asset_type: NewAssetType) -> Result<AssetType, DieselError> {
        let conn = &mut self.connection.get_connection()
            .map_err(|e| DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(e.to_string()),
            ))?;
        
        conn.transaction(|conn| {
            // Check if name already exists
            let exists = AssetTypeTable::table
                .filter(AssetTypeTable::name.eq(&new_asset_type.name))
                .count()
                .get_result::<i64>(conn)?;

            if exists > 0 {
                return Err(DieselError::NotFound);
            }

            // Insert new asset type
            let now = chrono::Utc::now().to_rfc3339();
            diesel::insert_into(AssetTypeTable::table)
                .values((
                    AssetTypeTable::name.eq(new_asset_type.name),
                    AssetTypeTable::created_at.eq(&now),
                    AssetTypeTable::updated_at.eq(&now),
                ))
                .get_result::<AssetType>(conn)
        })
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<AssetType>, DieselError> {
        let conn = &mut self.connection.get_connection()
            .map_err(|e| DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(e.to_string()),
            ))?;
        
        AssetTypeTable::table
            .filter(AssetTypeTable::id.eq(id))
            .first::<AssetType>(conn)
            .optional()
    }

    async fn get_all(&self) -> Result<Vec<AssetType>, DieselError> {
        let conn = &mut self.connection.get_connection()
            .map_err(|e| DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(e.to_string()),
            ))?;
        
        AssetTypeTable::table
            .order_by(AssetTypeTable::name)
            .load::<AssetType>(conn)
    }

    async fn update(&self, id: i32, asset_type: NewAssetType) -> Result<AssetType, DieselError> {
        let conn = &mut self.connection.get_connection()
            .map_err(|e| DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(e.to_string()),
            ))?;
        
        conn.transaction(|conn| {
            // Check if asset type exists
            let exists = AssetTypeTable::table
                .filter(AssetTypeTable::id.eq(id))
                .count()
                .get_result::<i64>(conn)?;

            if exists == 0 {
                return Err(DieselError::NotFound);
            }

            // Check if name already exists for other records
            let name_exists = AssetTypeTable::table
                .filter(AssetTypeTable::name.eq(&asset_type.name))
                .filter(AssetTypeTable::id.ne(id))
                .count()
                .get_result::<i64>(conn)?;

            if name_exists > 0 {
                return Err(DieselError::NotFound);
            }

            // Update the asset type
            let now = chrono::Utc::now().to_rfc3339();
            diesel::update(AssetTypeTable::table)
                .filter(AssetTypeTable::id.eq(id))
                .set((
                    AssetTypeTable::name.eq(asset_type.name),
                    AssetTypeTable::updated_at.eq(&now),
                ))
                .get_result::<AssetType>(conn)
        })
    }

    async fn delete(&self, id: i32) -> Result<bool, DieselError> {
        let conn = &mut self.connection.get_connection()
            .map_err(|e| DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(e.to_string()),
            ))?;
        
        // Check if asset type exists
        let exists = AssetTypeTable::table
            .filter(AssetTypeTable::id.eq(id))
            .count()
            .get_result::<i64>(conn)?;

        if exists == 0 {
            return Err(DieselError::NotFound);
        }

        let deleted = diesel::delete(AssetTypeTable::table)
            .filter(AssetTypeTable::id.eq(id))
            .execute(conn)?;

        Ok(deleted > 0)
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<AssetType>, DieselError> {
        let conn = &mut self.connection.get_connection()
            .map_err(|e| DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(e.to_string()),
            ))?;
        
        AssetTypeTable::table
            .filter(AssetTypeTable::name.like(format!("%{}%", name)))
            .order_by(AssetTypeTable::name)
            .load::<AssetType>(conn)
    }

    async fn exists(&self, id: i32) -> Result<bool, DieselError> {
        let conn = &mut self.connection.get_connection()
            .map_err(|e| DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(e.to_string()),
            ))?;
        
        let count = AssetTypeTable::table
            .filter(AssetTypeTable::id.eq(id))
            .count()
            .get_result::<i64>(conn)?;

        Ok(count > 0)
    }

    async fn count(&self) -> Result<i64, DieselError> {
        let conn = &mut self.connection.get_connection()
            .map_err(|e| DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(e.to_string()),
            ))?;
        
        AssetTypeTable::table
            .count()
            .get_result::<i64>(conn)
    }
} 