use actix_web::{error::ErrorUnauthorized, web, Error as ActixError, FromRequest};
use chrono::{Duration, Utc};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::dto::ApiResponse;
use crate::entities::user;
use crate::entities::sea_orm_active_enums::UserRole;

const JWT_SECRET: &[u8] = b"your-secret-key"; // In production, use environment variable
const TOKEN_EXPIRATION_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user id
    pub email: String,
    pub role: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
}

pub struct AuthMiddleware {
    pub user_id: i32,
    pub email: String,
    pub role: String,
}

impl FromRequest for AuthMiddleware {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            Some(header) => header,
            None => return err(ErrorUnauthorized("No authorization header")),
        };

        let auth_str = match auth_header.to_str() {
            Ok(str) => str,
            Err(_) => return err(ErrorUnauthorized("Invalid authorization header")),
        };

        if !auth_str.starts_with("Bearer ") {
            return err(ErrorUnauthorized("Invalid authorization header format"));
        }

        let token = &auth_str[7..];
        match validate_token(token) {
            Ok(claims) => ok(AuthMiddleware {
                user_id: claims.sub,
                email: claims.email,
                role: claims.role,
            }),
            Err(_) => err(ErrorUnauthorized("Invalid token")),
        }
    }
}

pub fn generate_token(user: &user::Model) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(TOKEN_EXPIRATION_HOURS))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        role: match user.user_role {
            UserRole::Admin => "Admin",
            UserRole::User => "User",
            UserRole::Moderator => "Moderator",
        }.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub async fn login(
    credentials: web::Json<LoginCredentials>,
    repo: web::Data<crate::repository::Repository>,
) -> Result<web::Json<ApiResponse<AuthResponse>>, ActixError> {
    // In a real application, you would hash the password and compare with stored hash
    let user = match repo
        .find_user_by_email(&credentials.email)
        .await
        .map_err(|_| ErrorUnauthorized("Invalid credentials"))?
    {
        Some(user) => user,
        None => return Err(ErrorUnauthorized("Invalid credentials")),
    };

    // TODO: Add proper password verification here
    // For now, we're just generating a token without password check
    let token = generate_token(&user).map_err(|_| ErrorUnauthorized("Token generation failed"))?;

    Ok(web::Json(ApiResponse::success(
        AuthResponse {
            token,
            token_type: "Bearer".to_string(),
        },
        "Login successful",
    )))
}
