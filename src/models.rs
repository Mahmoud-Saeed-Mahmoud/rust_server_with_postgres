use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    ADMIN,
    USER,
    MODERATOR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostStatus {
    DRAFT,
    PUBLISHED,
    ARCHIVED,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_role: UserRole,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: Option<i32>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub user_id: i32,
    pub phone_number: Option<String>,
    pub birth_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub post_status: PostStatus,
    pub view_count: i32,
    pub author_id: i32,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<i32>,
    pub category_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Option<i32>,
    pub content: String,
    pub post_id: i32,
    pub created_at: Option<DateTime<Utc>>,
}
