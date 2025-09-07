mod core;
mod santa;

use crate::core::backround_tasks::{delete_old_account_sessions, delete_old_auth_codes};
use crate::core::config::{AppConfig};
use crate::core::controllers::{auth_router, check_auth, hello, invite_router, ping, sign_up, user_router};
use crate::core::data_model::traits::ILocalObject;
use crate::core::functions::generate_id;
use crate::core::services::{create_role, create_roles_user_info, is_role_already_exists_by_name, row_to_account, row_to_invite, row_to_role, row_to_roles_user_info, user_sign_up, IDbService, SQLiteDbService};
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
use std::env;
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
        .route("/sign_up", get(spa_handler))
        .route("/login", get(spa_handler))
        .route("/profile", get(spa_handler))
        .route("/pools", get(spa_handler))
        .route("/chats", get(spa_handler))
        .route("/logout", get(spa_handler))
        .route("/create_pool", get(spa_handler));
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

    init_database(&state).await;
    install_default_drivers();

    //TODO: FOR TEST ONLY. REPLACE WITH ENV VARS WHEN AUTH 2.0 WILL END
    let db_service = SQLiteDbService::new(&state);
    
    let temp_code = "START UP";

    let is_invite_code_exists = db_service.exists_by_prop("invites", "invite_code", temp_code).await;
    if is_invite_code_exists.is_some_and(|b| {!b}) {
        let new_id_string = db_service.new_id("invites").await.unwrap();
        let new_id = new_id_string.as_str();
        let props = vec!["id", "invite_code", "one_use"];
        let values = vec![vec![new_id, temp_code, "true"]];
        let _ = db_service.insert("invites", props, values).await;
    }

    let is_admin_exists = is_account_already_exists_by_login("admin", &state).await;
    if is_admin_exists.is_some_and(|b| {!b}) {
        user_sign_up("admin", "qwerty123456", "qwerty123456", "BigBoss", "admin@test.ru", temp_code, &state).await;
        let admin = db_service.get_one_by_prop("accounts", "login", "admin", row_to_account).await.unwrap();
        let admin_role = db_service.get_one_by_prop("roles", "name", "administrator", row_to_role).await.unwrap();
        let roles_user_info_id = db_service.new_id("roles_user_infos").await.unwrap();
        let _ = db_service.delete_one_by_prop("roles_user_infos", "account_id", admin.id()).await;
        create_roles_user_info(roles_user_info_id.as_str(), admin.id(), admin_role.id(), "", &state).await;
    }

    let is_client_already_exists = is_client_already_exists_by_client_name("api", &state).await;
    if !is_client_already_exists.is_some_and(|b| {b}) {
        create_client(generate_id().await.as_str(), "api", "qwerty", "http://localhost:8000/oauth_code_redirect", true, &state).await;
    }

    let message_id = db_service.new_id("messages").await.unwrap();
    let now_time = Utc::now().to_rfc3339();
    
    let props = vec!["id", "text_content", "account_id", "pool_id", "room_id",  "creation_date"];
    let values = vec![vec![message_id.as_str(), "Hello world", "0", "0", "0", now_time.as_str()]];
    db_service.insert("messages", props, values).await;
    db_service.update("messages", "id", &message_id.as_str(), vec!["text_content"], vec!["Goodbye world"]).await;

    let exists_opt = db_service.exists_by_prop("messages", "id", message_id.as_str()).await;
    if exists_opt.is_some_and(|x| { x }) {
        let _ = db_service.get_one_by_prop("messages", "id", message_id.as_str(), row_to_message).await.unwrap();
        db_service.delete_one_by_prop("messages", "id", message_id.as_str()).await;
        let _ = db_service.exists_by_prop("messages", "id", message_id.as_str()).await;
    }


    // END TODO
    
    // start threads

    let state_clone = state.clone();

    tokio::spawn(async move { run_background_tasks(&state_clone).await });
    run_server(&state).await;
}