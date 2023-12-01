use sea_orm_migration::prelude::*;

use crate::{m20231127_011926_create_user_table::Users, m20231127_152600_chores::Chores};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserChoreAssignments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserChoreAssignments::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserChoreAssignments::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(UserChoreAssignments::Table)
                            .from_col(UserChoreAssignments::UserId)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id),
                    )
                    .col(
                        ColumnDef::new(UserChoreAssignments::ChoreId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(UserChoreAssignments::Table)
                            .from_col(UserChoreAssignments::ChoreId)
                            .to_tbl(Chores::Table)
                            .to_col(Chores::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserChoreAssignments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserChoreAssignments {
    Table,
    Id,
    UserId,
    ChoreId,
}
