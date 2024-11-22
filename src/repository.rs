use sea_orm::*;
use crate::entities::{user, profile, post};
use chrono::{DateTime, FixedOffset, Utc};
use crate::dto::UserCreateDto;

pub struct Repository {
    db: DatabaseConnection,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // Helper function to get current time as DateTime<FixedOffset>
    fn now() -> DateTime<FixedOffset> {
        Utc::now().into()
    }

    // Helper function to convert NaiveDateTime to DateTime<Utc>
    fn to_utc(date: Option<DateTime<Utc>>) -> Option<DateTime<Utc>> {
        date
    }

    // User operations
    pub async fn find_user_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_all_users(&self) -> Result<Vec<user::Model>, DbErr> {
        user::Entity::find().all(&self.db).await
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await
    }

    pub async fn create_user(&self, user_data: UserCreateDto) -> Result<user::Model, DbErr> {
        let now = Utc::now();
        let user = user::ActiveModel {
            email: Set(user_data.email),
            first_name: Set(user_data.first_name),
            last_name: Set(user_data.last_name),
            user_role: Set(user_data.user_role),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        user.insert(&self.db).await
    }

    pub async fn update_user(&self, id: i32, user_data: user::Model) -> Result<user::Model, DbErr> {
        let user = user::ActiveModel {
            id: Set(id),
            email: Set(user_data.email),
            first_name: Set(user_data.first_name),
            last_name: Set(user_data.last_name),
            user_role: Set(user_data.user_role),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        user.update(&self.db).await
    }

    pub async fn delete_user(&self, id: i32) -> Result<DeleteResult, DbErr> {
        let user = user::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(user)
    }

    // Profile operations
    pub async fn find_profile_by_id(&self, id: i32) -> Result<Option<profile::Model>, DbErr> {
        profile::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_all_profiles(&self) -> Result<Vec<profile::Model>, DbErr> {
        profile::Entity::find().all(&self.db).await
    }

    pub async fn create_profile(&self, profile_data: profile::Model) -> Result<profile::Model, DbErr> {
        let profile = profile::ActiveModel {
            bio: Set(profile_data.bio),
            avatar: Set(profile_data.avatar),
            user_id: Set(profile_data.user_id),
            phone_number: Set(profile_data.phone_number),
            birth_date: Set(Self::to_utc(profile_data.birth_date)),
            ..Default::default()
        };

        profile.insert(&self.db).await
    }

    pub async fn update_profile(&self, id: i32, profile_data: profile::Model) -> Result<profile::Model, DbErr> {
        let profile = profile::ActiveModel {
            id: Set(id),
            bio: Set(profile_data.bio),
            avatar: Set(profile_data.avatar),
            user_id: Set(profile_data.user_id),
            phone_number: Set(profile_data.phone_number),
            birth_date: Set(Self::to_utc(profile_data.birth_date)),
        };

        profile.update(&self.db).await
    }

    pub async fn delete_profile(&self, id: i32) -> Result<DeleteResult, DbErr> {
        let profile = profile::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(profile)
    }

    // Post operations
    pub async fn find_post_by_id(&self, id: i32) -> Result<Option<post::Model>, DbErr> {
        post::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_all_posts(&self) -> Result<Vec<post::Model>, DbErr> {
        post::Entity::find().all(&self.db).await
    }

    pub async fn create_post(&self, post_data: post::Model) -> Result<post::Model, DbErr> {
        let now = Self::now();
        let post = post::ActiveModel {
            title: Set(post_data.title),
            content: Set(post_data.content),
            published: Set(post_data.published),
            author_id: Set(post_data.author_id),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        post.insert(&self.db).await
    }

    pub async fn update_post(&self, id: i32, post_data: post::Model) -> Result<post::Model, DbErr> {
        let post = post::ActiveModel {
            id: Set(id),
            title: Set(post_data.title),
            content: Set(post_data.content),
            published: Set(post_data.published),
            author_id: Set(post_data.author_id),
            updated_at: Set(Self::now()),
            ..Default::default()
        };

        post.update(&self.db).await
    }

    pub async fn delete_post(&self, id: i32) -> Result<DeleteResult, DbErr> {
        let post = post::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(post)
    }
}
