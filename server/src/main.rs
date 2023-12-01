extern crate dotenv;

use std::env;

use axum::{routing::post, Router};
use dotenv::dotenv;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use sea_orm::{Database, DatabaseConnection};

use app::*;
use fileserv::file_and_error_handler;
use migration::{Migrator, MigratorTrait};

pub mod fileserv;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("Couldn't initialize logging");
    dotenv().ok();

    // Setup database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection: DatabaseConnection = Database::connect(database_url).await.unwrap();
    Migrator::up(&connection, None).await.unwrap();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
