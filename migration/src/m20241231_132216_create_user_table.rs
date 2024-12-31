use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::UserId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::CognitoId).string().not_null())
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::ProfilePictureUrl).string())
                    .col(ColumnDef::new(User::TeamId).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_cognito_id")
                    .table(User::Table)
                    .col(User::CognitoId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_username")
                    .table(User::Table)
                    .col(User::Username)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    UserId,
    CognitoId,
    Username,
    ProfilePictureUrl,
    TeamId,
}
