#[derive(Iden)]
enum UserRole {
    #[iden = "user_role"]
    Type,
}

use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create UserRole enum type
        let create_enum = Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE "user_role" AS ENUM ('ADMIN', 'USER', 'MODERATOR')"#.to_owned(),
        );
        manager.get_connection().execute(create_enum).await?;

        // Create User table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::FirstName).string().not_null())
                    .col(ColumnDef::new(User::LastName).string().not_null())
                    .col(
                        ColumnDef::new(User::UserRole)
                            .custom(UserRole::Type)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Profile table
        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .col(
                        ColumnDef::new(Profile::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Profile::UserId).integer().not_null())
                    .col(ColumnDef::new(Profile::Bio).string())
                    .col(ColumnDef::new(Profile::Avatar).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-profile-user_id")
                            .from(Profile::Table, Profile::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Post table
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Content).text().not_null())
                    .col(ColumnDef::new(Post::Published).boolean().not_null().default(false))
                    .col(ColumnDef::new(Post::AuthorId).integer().not_null())
                    .col(
                        ColumnDef::new(Post::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Post::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-author_id")
                            .from(Post::Table, Post::AuthorId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        // Drop the enum type
        let drop_enum = Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS "user_role""#.to_owned(),
        );
        manager.get_connection().execute(drop_enum).await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Email,
    FirstName,
    LastName,
    UserRole,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Profile {
    Table,
    Id,
    UserId,
    Bio,
    Avatar,
}

#[derive(Iden)]
pub enum Post {
    Table,
    Id,
    Title,
    Content,
    Published,
    AuthorId,
    CreatedAt,
    UpdatedAt,
}
