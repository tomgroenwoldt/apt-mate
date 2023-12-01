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
                    .table(Expenses::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Expenses::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Expenses::Cost).float().not_null())
                    .col(ColumnDef::new(Expenses::Description).string())
                    .col(ColumnDef::new(Expenses::CreatedBy).uuid().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Expenses::Table)
                            .from_col(Expenses::CreatedBy)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id),
                    )
                    .col(ColumnDef::new(Expenses::Status).string())
                    .col(ColumnDef::new(Expenses::SwapRequested).boolean())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Expenses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Expenses {
    Table,
    Id,
    Cost,
    Description,
    CreatedBy,
    Status,
    SwapRequested,
}
