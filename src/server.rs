use actix_web::{web, App, HttpServer};
use sea_orm::DatabaseConnection;
use crate::api::*;

pub async fn start_server(db: DatabaseConnection) -> std::io::Result<()> {
    println!("Starting server at http://localhost:3333");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
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
    .bind(("127.0.0.1", 3333))?
    .run()
    .await
}
