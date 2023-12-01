use entities::{chores, expenses, user_chore_assignments, user_expense_assignments, users};
use rand::{seq::SliceRandom, thread_rng, Rng};
use sea_orm_migration::{
    prelude::*,
    sea_orm::{prelude::Uuid, ActiveModelTrait, Set},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Create some users
        let user_data = [
            ("Tom", "Groenwoldt", "Shilling"),
            ("Lars", "Rosendorf", "In VR"),
            ("Leif", "Schildwert", "Am Fliegen"),
        ];
        let mut users = vec![];
        for (first_name, last_name, status) in user_data {
            let user = users::ActiveModel {
                id: Set(Uuid::new_v4()),
                first_name: Set(first_name.to_owned()),
                last_name: Set(last_name.to_owned()),
                picture: Set(None),
                status: Set(status.to_owned()),
            }
            .insert(db)
            .await?;
            users.push(user);
        }

        // Create some chores
        let chore_data = ["Badezimmer", "Flur", "Kueche"];
        let mut chores = vec![];
        for (user, chore) in users.iter().zip(chore_data) {
            let chore = chores::ActiveModel {
                id: Set(Uuid::new_v4()),
                name: Set(chore.to_owned()),
                time_frame: Set(604800),
                created_by: Set(user.id),
                status: Set("TODO".to_owned()),
                swap_requested: Set(false),
            }
            .insert(db)
            .await?;
            chores.push(chore);
        }

        // Now, assign chores to users
        for (user, chore) in users.iter().zip(chores) {
            user_chore_assignments::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user.id),
                chore_id: Set(chore.id),
            }
            .insert(db)
            .await?;
        }

        // Create some expenses
        let expense_data = [("Milch", 1.99), ("Oel", 5.99), ("Toilettenpapier", 2.99)];
        let mut expenses = vec![];
        for (user, expense) in users.iter().zip(expense_data) {
            let (description, cost) = expense;
            let expense = expenses::ActiveModel {
                id: Set(Uuid::new_v4()),
                cost: Set(cost),
                description: Set(Some(description.to_owned())),
                created_by: Set(user.id),
                status: Set(Some("Offen".to_owned())),
                swap_requested: Set(Some(false)),
            }
            .insert(db)
            .await?;
            expenses.push(expense);
        }

        // Now, assign expenses to users
        for expense in expenses {
            let user_count = thread_rng().gen_range(1..=3);
            let random_users = users
                .choose_multiple(&mut thread_rng(), user_count)
                .collect::<Vec<_>>();
            for user in random_users {
                user_expense_assignments::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    user_id: Set(user.id),
                    expense_id: Set(expense.id),
                }
                .insert(db)
                .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
