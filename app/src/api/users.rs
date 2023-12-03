use cfg_if::cfg_if;
use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct FrontendUser {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub picture: Option<Vec<u8>>,
    pub status: String,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sea_orm::{DatabaseConnection, EntityTrait};
        use entities::users::{Entity, Model as User};

        impl From<User> for FrontendUser {
            fn from(user: User) -> FrontendUser {
                FrontendUser {
                    id: user.id,
                    first_name: user.first_name,
                    last_name: user.last_name,
                    picture: user.picture,
                    status: user.status
                }
            }
        }

        pub async fn fetch_users(connection: &DatabaseConnection) -> Result<Vec<FrontendUser>, ServerFnError> {
            let users = Entity::find().all(connection).await?;
            let frontend_users = users.into_iter().map(FrontendUser::from).collect::<Vec<_>>();
            Ok(frontend_users)
        }
    }
}

#[server(GetUsers)]
pub async fn get_users() -> Result<Vec<FrontendUser>, ServerFnError> {
    use crate::pool;

    let connection = pool()?;
    let users = fetch_users(&connection).await?;

    Ok(users)
}
