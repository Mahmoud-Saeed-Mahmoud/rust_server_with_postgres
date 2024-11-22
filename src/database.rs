use sea_orm::{Database, DbConn, Statement, DbErr, ConnectionTrait};
use std::env;

pub async fn create_pool() -> Result<DbConn, DbErr> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Get the database URL from environment variables
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Create connection pool
    Database::connect(&database_url).await
}

// Function to test the database connection
pub async fn test_connection(db: &DbConn) -> Result<(), DbErr> {
    // Try to get a connection and execute a simple query
    let result = db
        .execute(Statement::from_string(
            db.get_database_backend(),
            "SELECT 1".to_owned(),
        ))
        .await?;

    println!("Database connection test successful! Rows affected: {}", result.rows_affected());
    Ok(())
}
