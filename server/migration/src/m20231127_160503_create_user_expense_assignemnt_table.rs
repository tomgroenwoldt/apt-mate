use sea_orm_migration::prelude::*;

use crate::{m20231127_011926_create_user_table::Users, m20231127_152607_expenses::Expenses};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserExpenseAssignments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserExpenseAssignments::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserExpenseAssignments::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(UserExpenseAssignments::Table)
                            .from_col(UserExpenseAssignments::UserId)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id),
                    )
                    .col(
                        ColumnDef::new(UserExpenseAssignments::ExpenseId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(UserExpenseAssignments::Table)
                            .from_col(UserExpenseAssignments::ExpenseId)
                            .to_tbl(Expenses::Table)
                            .to_col(Expenses::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(UserExpenseAssignments::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserExpenseAssignments {
    Table,
    Id,
    UserId,
    ExpenseId,
}
