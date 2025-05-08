use diesel::r2d2::{self, ConnectionManager, Pool};
use std::env;
use dotenv::dotenv;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Failed to load environment variables: {0}")]
    EnvError(#[from] env::VarError),
    
    #[error("Failed to create connection pool: {0}")]
    PoolError(String),
    
    #[error("Failed to establish database connection: {0}")]
    ConnectionError(#[from] diesel::ConnectionError),
}

pub struct SqliteConnection {
    pool: Pool<ConnectionManager<diesel::SqliteConnection>>,
}

impl SqliteConnection {
    pub fn new() -> Result<Self, ConnectionError> {
        // Load environment variables from .env file
        dotenv().ok();
        
        // Get database URL from environment variable
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "banking.db".to_string());
        
        // Create connection manager
        let manager = ConnectionManager::<diesel::SqliteConnection>::new(&database_url);
        
        // Create connection pool
        let pool = match Pool::builder()
            .max_size(15) // Maximum number of connections in the pool
            .build(manager) {
                Ok(pool) => pool,
                Err(e) => return Err(ConnectionError::PoolError(e.to_string())),
            };
        
        Ok(Self { pool })
    }
    
    pub fn get_connection(&self) -> r2d2::PooledConnection<ConnectionManager<diesel::SqliteConnection>> {
        self.pool.get().expect("Failed to get connection from pool")
    }
    
    pub fn get_pool(&self) -> &Pool<ConnectionManager<diesel::SqliteConnection>> {
        &self.pool
    }
}

// Implement Clone for SqliteConnection
impl Clone for SqliteConnection {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

// Implement Send and Sync for SqliteConnection
unsafe impl Send for SqliteConnection {}
unsafe impl Sync for SqliteConnection {}
