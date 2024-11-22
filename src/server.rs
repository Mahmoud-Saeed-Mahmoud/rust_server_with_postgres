use actix_web::{web, App, HttpServer};
use sea_orm::DatabaseConnection;
use std::env;
use crate::api::*;

pub async fn start_server(db: DatabaseConnection) -> std::io::Result<()> {
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3333".to_string());
    let workers = env::var("SERVER_WORKERS")
        .unwrap_or_else(|_| num_cpus::get().to_string())
        .parse::<usize>()
        .unwrap_or_else(|_| num_cpus::get());

    println!("Starting server at http://{}:{}", host, port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .service(
                web::scope("/api")
                    .service(web::scope("/users")
                        .route("", web::get().to(get_users))
                        .route("", web::post().to(create_user))
                        .route("/{id}", web::get().to(get_user))
                        .route("/{id}", web::put().to(update_user))
                        .route("/{id}", web::delete().to(delete_user)))
                    .service(web::scope("/profiles")
                        .route("", web::get().to(get_profiles))
                        .route("", web::post().to(create_profile))
                        .route("/{id}", web::get().to(get_profile))
                        .route("/{id}", web::put().to(update_profile))
                        .route("/{id}", web::delete().to(delete_profile)))
                    .service(web::scope("/posts")
                        .route("", web::get().to(get_posts))
                        .route("", web::post().to(create_post))
                        .route("/{id}", web::get().to(get_post))
                        .route("/{id}", web::put().to(update_post))
                        .route("/{id}", web::delete().to(delete_post)))
            )
    })
    .workers(workers)
    .bind((host, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
