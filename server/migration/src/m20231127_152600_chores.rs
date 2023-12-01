use sea_orm_migration::prelude::*;

use crate::m20231127_011926_create_user_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chores::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Chores::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Chores::Name).string().not_null())
                    .col(ColumnDef::new(Chores::TimeFrame).big_unsigned().not_null())
                    .col(ColumnDef::new(Chores::CreatedBy).uuid().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Chores::Table)
                            .from_col(Chores::CreatedBy)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id),
                    )
                    .col(ColumnDef::new(Chores::Status).string().not_null())
                    .col(ColumnDef::new(Chores::SwapRequested).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chores::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Chores {
    Table,
    Id,
    Name,
    TimeFrame,
    CreatedBy,
    Status,
    SwapRequested,
}
