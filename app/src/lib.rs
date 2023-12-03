use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::api::users::get_users;

pub mod api;
pub mod error_template;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sea_orm::DatabaseConnection;

        pub fn pool() -> Result<DatabaseConnection, ServerFnError> {
            use_context::<DatabaseConnection>()
                .ok_or_else(|| ServerFnError::ServerError("Database connection missing.".into()))

        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Apt mate"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let users = create_resource(|| (), |_| async move { get_users().await });

    view! {
        <Transition
            fallback=move || view! { <p>"Loading..."</p> }
        >
            {move || match users.get() {
                None => None,
                Some(Err(_)) => Some(view! { <p>"Error loading stories."</p> }.into_any()),
                Some(Ok(users)) => {
                    Some(view! {
                        <ul>
                            <For
                                each=move || users.clone()
                                key=|user| user.first_name.clone()
                                let:user
                            >
                                {user.first_name}
                            </For>
                        </ul>
                    }.into_any())
                }
            }}
        </Transition>
    }
}
