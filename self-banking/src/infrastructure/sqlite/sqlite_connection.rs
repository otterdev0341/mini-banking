use diesel::prelude::*;
use diesel::r2d2::{self, Pool, PooledConnection, ConnectionManager};
use r2d2::Error as R2d2Error;
use std::env;
use dotenv::dotenv;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct SqliteConnection {
    pool: Pool<ConnectionManager<diesel::SqliteConnection>>,
}

impl SqliteConnection {
    pub fn new() -> Result<Self, ConnectionError> {
        // Load environment variables from .env file
        dotenv().ok();
        
        // Get database URL from environment variable
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConnectionError::EnvVarError("DATABASE_URL not set".to_string()))?;
        
        // Create connection manager
        let manager = ConnectionManager::<diesel::SqliteConnection>::new(&database_url);
        
        // Create connection pool
        let pool = Pool::builder()
            .max_size(15) // Maximum number of connections in the pool
            .build(manager)
            .map_err(|e| ConnectionError::PoolError(diesel::r2d2::Error::ConnectionError(diesel::ConnectionError::BadConnection(e.to_string()))))?;
        
        Ok(Self { pool })
    }
    
    pub fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<diesel::SqliteConnection>>, diesel::r2d2::Error> {
        self.pool.get().map_err(|e| diesel::r2d2::Error::ConnectionError(diesel::ConnectionError::BadConnection(e.to_string())))
    }
    
    pub fn get_pool(&self) -> &Pool<ConnectionManager<diesel::SqliteConnection>> {
        &self.pool
    }
}

// Implement Send and Sync for SqliteConnection
unsafe impl Send for SqliteConnection {}
unsafe impl Sync for SqliteConnection {}

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Environment variable error: {0}")]
    EnvVarError(String),
    
    #[error("Connection pool error: {0}")]
    PoolError(#[from] diesel::r2d2::Error),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
}
