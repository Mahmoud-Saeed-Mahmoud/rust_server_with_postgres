use serde::{Deserialize, Serialize};
use crate::entities::sea_orm_active_enums::UserRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateDto {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_role: UserRole,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            status: "success".to_string(),
            code: 200,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: u16, message: &str) -> Self {
        Self {
            status: "error".to_string(),
            code,
            message: message.to_string(),
            data: None,
        }
    }
}
