use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table =  Table::create()
            .table(User::Table)
            .if_not_exists()
            .col(pk_auto(User::UserId).auto_increment())
            .col(string(User::FirstName))
            .col(string(User::LastName))
            .col(string(User::Email))
            .col(string_uniq(User::Password))
            .col(date(User::BirthDate))
            .col(decimal(User::Weight))
            .col(decimal(User::Height))
            .col(string(User::Gender))
            .col(string(User::HealthGoal))
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "users")]
    Table,
    UserId,
    FirstName,
    LastName,
    Email,
    Password,
    BirthDate,
    Weight,
    Height,
    Gender,
    HealthGoal,
}
