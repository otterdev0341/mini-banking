use dotenv::dotenv;

pub struct SqliteConfig{
    pub path: String
}


impl Default for SqliteConfig {
    fn default() -> Self {
        dotenv().ok();
        let path = match std::env::var("SQLITE_PATH") {
            Ok(path) => path,
            Err(_) => {
                println!("Database path not Found");
                format!("not found database path")
            }
        };
        Self { path }
    }
}