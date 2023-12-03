extern crate dotenv;

use std::env;

use app::App;
use axum::{
    body::Body,
    extract::{FromRef, Path, RawQuery, State},
    http::{HeaderMap, Request},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use dotenv::dotenv;
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use leptos_router::RouteListing;
use sea_orm::{Database, DatabaseConnection};

use fileserv::file_and_error_handler;
use migration::{Migrator, MigratorTrait};

pub mod fileserv;

#[derive(FromRef, Debug, Clone)]
struct AppState {
    connection: DatabaseConnection,
    leptos_options: LeptosOptions,
    routes: Vec<RouteListing>,
}

async fn server_fn_handler(
    State(app_state): State<AppState>,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<Body>,
) -> impl IntoResponse {
    log::info!("{:?}", path);

    handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move || {
            provide_context(app_state.connection.clone());
        },
        request,
    )
    .await
}

async fn leptos_routes_handler(State(app_state): State<AppState>, req: Request<Body>) -> Response {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            provide_context(app_state.connection.clone());
        },
        App,
    );
    handler(req).await.into_response()
}

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
    let app_state = AppState {
        connection,
        leptos_options,
        routes: routes.clone(),
    };
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
