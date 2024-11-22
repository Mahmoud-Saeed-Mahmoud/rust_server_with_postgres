use sea_orm::{Database, DbConn, Statement, ConnectionTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the default database
    let db: DbConn = Database::connect("postgres://postgres:123456@localhost:5432/postgres").await?;
    
    // Create the new database
    let create_db = Statement::from_string(
        db.get_database_backend(),
        "CREATE DATABASE crud_from_prisma".to_owned(),
    );

    db.execute(create_db).await?;
    println!("Database created successfully!");
    
    Ok(())
}
