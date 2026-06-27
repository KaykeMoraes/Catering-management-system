use std::env;

use dotenvy::dotenv;
use sqlx::PgPool;

pub struct DatabaseConnection {
    pool: PgPool,
}

impl DatabaseConnection {
    pub fn init() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Failed to load DATABASE_URL variable");
        let connection = PgPool::connect_lazy(&database_url);
        if connection.is_err() {
            error!(
                "Failed to get connection with the database: {}",
                connection.unwrap_err()
            );
            panic!("Failed to connect to database");
        }
        Self {
            pool: connection.unwrap(),
        }
    }
}
