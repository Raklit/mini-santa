mod core;
mod santa;

use crate::core::background_tasks::{delete_old_account_sessions, delete_old_auth_codes};
use crate::core::config::{AppConfig};
use crate::core::controllers::{auth_router, check_auth, hello, invite_router, ping, sign_up, user_router};
use crate::core::data_model::traits::ILocalObject;
use crate::core::functions::{generate_id, generate_random_token};
use crate::core::services::{create_roles_user_info, init_admin_if_not_exists, row_to_account, row_to_role, user_sign_up, IDbService, SQLiteDbService};
use crate::santa::background_tasks::{delete_old_messages, delete_old_pools};
use crate::santa::controllers::santa_router;
use crate::santa::functions::santa_init_database;
use crate::santa::services::row_to_message;

use axum::middleware::from_fn_with_state;
use axum::routing::post;
use axum:: {
    routing::get,
    Router,
    extract::State,
    response::Html
};

use chrono::Utc;
use sqlx::any::install_default_drivers;
use sqlx::SqlitePool;
use tokio::sync::Mutex;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, Layer};
use tracing_subscriber::layer::SubscriberExt;
use core::functions::core_init_database;
use core::services::{create_client, is_account_already_exists_by_login, is_client_already_exists_by_client_name};
use std::fs::{self, OpenOptions};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::{DefaultOnResponse, DefaultMakeSpan, TraceLayer};
use tracing::Level;
use tower_http::services::ServeDir;
use tera::{Tera, Context};
use sqlx::{migrate::MigrateDatabase, Sqlite};

#[derive(Clone)]
pub struct AppState {
    tera : Arc<Mutex<Tera>>,
    context : Arc<Mutex<Context>>,
    db : Arc<Mutex<SqlitePool>>,
    config: Arc<Mutex<AppConfig>>
}

async fn init_database(state : &AppState) {
    core_init_database(state).await;
    santa_init_database(state).await;
}

async fn run_background_tasks(state : &AppState) -> () {
    delete_old_account_sessions(state).await;
    delete_old_auth_codes(state).await;
    delete_old_messages(state).await;
    delete_old_pools(state).await;
}

// routers groups
pub fn no_auth_api_router() -> Router<AppState> {
    return Router::new()
        .route("/sign_up", post(sign_up))
        .route("/ping", get(ping))
        .route("/hello", get(hello));
}

pub fn ui_router() -> Router<AppState> {
    return Router::new()
        .route("/", get(spa_handler))
        .route("/login", get(spa_handler))
        .route("/logout", get(spa_handler))
        .route("/sign_up", get(spa_handler))
        .route("/invite_codes", get(spa_handler))
        .route("/create_invite_code", get(spa_handler))
        .route("/pools", get(spa_handler))
        .route("/pools/id/{id}", get(spa_handler))
        .route("/pools/id/{id}/add", get(spa_handler))
        .route("/create_pool", get(spa_handler))
        .route("/pools/id/{id}/delete_pool", get(spa_handler))
        .route("/chats", get(spa_handler))
        .route("/chats/id/{id}", get(spa_handler))
        .route("/profile", get(spa_handler))
}

pub fn need_auth_api_router(state : AppState) -> Router<AppState> {
    return Router::new()
        .nest("/users", user_router(&state))
        .nest("/invites", invite_router(&state))
        .nest("/santa", santa_router(&state))
        .layer(from_fn_with_state(state, check_auth))
}

pub fn api_router(state : AppState) -> Router<AppState> {
    return need_auth_api_router(state).merge(no_auth_api_router());
}

async fn run_server(state : &AppState) {
    let app = Router::new()
        .nest("/api", api_router(state.clone()))
        .nest("/oauth", auth_router())
        .merge(ui_router())
        .with_state(state.clone())
        .nest_service("/static", ServeDir::new("./app/static"))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn spa_handler(State(state) : State<AppState>) -> Html<String> {
    let mut new_context = tera::Context::new();
    new_context.extend(state.context.lock().await.clone());
    let temp = match state.tera.lock().await.render("index.html", &new_context) {
        Ok(s) => Html(s),
        Err(e) => {
            tracing::error!("Parsing error(s): {}", e);
            return Html(String::from("<p>Internal error</p>"));
        }
    };
    return temp;
}

#[tokio::main]
async fn main() {
    const CONFIG_PATH : &str = "./Config.toml";
    let raw_config = fs::read_to_string(CONFIG_PATH).expect("Can't read config file");
    let app_config : AppConfig = toml::from_str(raw_config.as_str()).unwrap();
    
    // enable logging

    let log_path = std::path::Path::new(&app_config.server.log_path);
    let _ = std::fs::create_dir_all(log_path.parent().unwrap());

    let log_file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(log_path)
    .unwrap();

    let std_out_layer = tracing_subscriber::fmt::layer()
    .with_target(false)
    .with_ansi(true)
    .compact()
    .with_filter(filter::LevelFilter::from_level(Level::INFO));

    let file_out_layer = tracing_subscriber::fmt::layer()
    .with_target(false)
    .with_ansi(false)
    .with_writer(log_file)
    .compact()
    .with_filter(filter::LevelFilter::from_level(Level::INFO));

    let log_layer = file_out_layer.and_then(std_out_layer);

    tracing_subscriber::registry().with(log_layer).init();


    // enable tera templates

    let mut tera = match Tera::new("./app/templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Parsing error(s): {}", e);
            ::std::process::exit(1);

        }
    };
    tera.autoescape_on(vec![".html", ".sql"]);

    let mut context = Context::new();
    context.insert("word", "ho-ho");


    let db_url = format!("sqlite://{}", &app_config.database.db_file);
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        let _ = Sqlite::create_database(&db_url).await;
    }

    let db = SqlitePool::connect(&db_url).await.unwrap();

    let state = AppState {
        tera: Arc::new(Mutex::new(tera)),
        context: Arc::new(Mutex::new(context)),
        db: Arc::new(Mutex::new(db)),
        config: Arc::new(Mutex::new(app_config))
    };

    // init database

    init_database(&state).await;
    install_default_drivers();


    // create admin account
    
    let cloned_state = state.clone();

    let resp = init_admin_if_not_exists(&cloned_state).await;
    let body_string = resp.body.to_string();
    let body = body_string.as_str();
    if resp.is_ok() {
        tracing::info!("{:?}", body)
    } else {
        tracing::error!("{:?}", body);
    }

    // create client

    let is_client_already_exists = is_client_already_exists_by_client_name("api", &state).await;
    if !is_client_already_exists.is_some_and(|b| {b}) {
        create_client(generate_id().await.as_str(), "api", "", "http://localhost:8000/oauth_code_redirect", true, &state).await;
    }
    
    // start threads

    tokio::spawn(async move { run_background_tasks(&cloned_state).await });
    
    // start server

    run_server(&state).await;
}