use crud_from_prisma::server::start_server;
use crud_from_prisma::database::create_pool;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Create database connection pool
    let db = create_pool()
        .await
        .expect("Failed to create database connection pool");

    // Start the server
    start_server(db).await
}