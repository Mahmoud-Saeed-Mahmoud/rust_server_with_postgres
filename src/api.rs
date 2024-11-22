use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use crate::entities::{user, profile, post};
use crate::repository::Repository;
use crate::dto::{UserCreateDto, ApiResponse};

// User handlers

pub async fn get_users(db: web::Data<DatabaseConnection>) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.find_all_users().await {
        Ok(users) => HttpResponse::Ok().json(ApiResponse::success(users, "Users retrieved successfully")),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<Vec<user::Model>>::error(500, &format!("Error retrieving users: {}", err))
        ),
    }
}

pub async fn create_user(db: web::Data<DatabaseConnection>, user_data: web::Json<UserCreateDto>) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.create_user(user_data.0).await {
        Ok(user) => HttpResponse::Created().json(ApiResponse::success(user, "User created successfully")),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<user::Model>::error(500, &format!("Error creating user: {}", err))
        ),
    }
}

pub async fn get_user(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.find_user_by_id(id.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse::success(user, "User found")),
        Ok(None) => HttpResponse::NotFound().json(
            ApiResponse::<user::Model>::error(404, "User not found")
        ),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<user::Model>::error(500, &format!("Error retrieving user: {}", err))
        ),
    }
}

pub async fn update_user(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
    user: web::Json<user::Model>,
) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.update_user(id.into_inner(), user.0).await {
        Ok(user) => HttpResponse::Ok().json(ApiResponse::success(user, "User updated successfully")),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<user::Model>::error(500, &format!("Error updating user: {}", err))
        ),
    }
}

pub async fn delete_user(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.delete_user(id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success((), "User deleted successfully")),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(500, &format!("Error deleting user: {}", err))
        ),
    }
}

// Profile handlers

pub async fn get_profiles(db: web::Data<DatabaseConnection>) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.find_all_profiles().await {
        Ok(profiles) => HttpResponse::Ok().json(ApiResponse::success(profiles, "Profiles retrieved successfully")),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<Vec<profile::Model>>::error(500, &format!("Error retrieving profiles: {}", err))
        ),
    }
}

pub async fn create_profile(db: web::Data<DatabaseConnection>, profile: web::Json<profile::Model>) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.create_profile(profile.0).await {
        Ok(profile) => HttpResponse::Created().json(ApiResponse::success(profile, "Profile created successfully")),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<profile::Model>::error(500, &format!("Error creating profile: {}", err))
        ),
    }
}

pub async fn get_profile(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.find_profile_by_id(id.into_inner()).await {
        Ok(Some(profile)) => HttpResponse::Ok().json(ApiResponse::success(profile, "Profile found")),
        Ok(None) => HttpResponse::NotFound().json(
            ApiResponse::<profile::Model>::error(404, "Profile not found")
        ),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<profile::Model>::error(500, &format!("Error retrieving profile: {}", err))
        ),
    }
}

pub async fn update_profile(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
    profile: web::Json<profile::Model>,
) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.update_profile(id.into_inner(), profile.0).await {
        Ok(profile) => HttpResponse::Ok().json(ApiResponse::success(profile, "Profile updated successfully")),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<profile::Model>::error(500, &format!("Error updating profile: {}", err))
        ),
    }
}

pub async fn delete_profile(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.delete_profile(id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success((), "Profile deleted successfully")),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(500, &format!("Error deleting profile: {}", err))
        ),
    }
}

// Post handlers

pub async fn get_posts(db: web::Data<DatabaseConnection>) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.find_all_posts().await {
        Ok(posts) => HttpResponse::Ok().json(ApiResponse::success(posts, "Posts retrieved successfully")),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<Vec<post::Model>>::error(500, &format!("Error retrieving posts: {}", err))
        ),
    }
}

pub async fn create_post(db: web::Data<DatabaseConnection>, post: web::Json<post::Model>) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.create_post(post.0).await {
        Ok(post) => HttpResponse::Created().json(ApiResponse::success(post, "Post created successfully")),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<post::Model>::error(500, &format!("Error creating post: {}", err))
        ),
    }
}

pub async fn get_post(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.find_post_by_id(id.into_inner()).await {
        Ok(Some(post)) => HttpResponse::Ok().json(ApiResponse::success(post, "Post found")),
        Ok(None) => HttpResponse::NotFound().json(
            ApiResponse::<post::Model>::error(404, "Post not found")
        ),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<post::Model>::error(500, &format!("Error retrieving post: {}", err))
        ),
    }
}

pub async fn update_post(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
    post: web::Json<post::Model>,
) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.update_post(id.into_inner(), post.0).await {
        Ok(post) => HttpResponse::Ok().json(ApiResponse::success(post, "Post updated successfully")),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<post::Model>::error(500, &format!("Error updating post: {}", err))
        ),
    }
}

pub async fn delete_post(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    let repo = Repository::new(db.get_ref().clone());
    match repo.delete_post(id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success((), "Post deleted successfully")),
        Err(err) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(500, &format!("Error deleting post: {}", err))
        ),
    }
}
