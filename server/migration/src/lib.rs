pub use sea_orm_migration::prelude::*;

mod m20231127_011926_create_user_table;
mod m20231127_152600_chores;
mod m20231127_152607_expenses;
mod m20231127_160503_create_user_expense_assignemnt_table;
mod m20231127_161328_create_user_chore_assignemnt_table;
mod m20231201_184318_seed;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231127_011926_create_user_table::Migration),
            Box::new(m20231127_152600_chores::Migration),
            Box::new(m20231127_152607_expenses::Migration),
            Box::new(m20231127_160503_create_user_expense_assignemnt_table::Migration),
            Box::new(m20231127_161328_create_user_chore_assignemnt_table::Migration),
            Box::new(m20231201_184318_seed::Migration),
        ]
    }
}
