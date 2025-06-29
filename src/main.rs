mod core;

use crate::core::backround_tasks::{delete_old_account_sessions, delete_old_auth_codes};
use crate::core::config::{AppConfig};
use crate::core::controllers::{api_router, auth_router};
use crate::core::functions::generate_id;
use crate::core::services::user_sign_up;

use axum:: {
    routing::get,
    Router,
    extract::State,
    response::Html
};

use sqlx::{SqlitePool};
use tokio::sync::Mutex;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, Layer};
use tracing_subscriber::layer::SubscriberExt;
use core::functions::init_database;
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
struct AppState {
    tera : Arc<Mutex<Tera>>,
    context : Arc<Mutex<Context>>,
    db : Arc<Mutex<SqlitePool>>,
    config: Arc<Mutex<AppConfig>>
}

async fn run_background_tasks(state : &AppState) -> () {
    delete_old_account_sessions(state).await;
    delete_old_auth_codes(state).await;
}

async fn run_server(state : &AppState) {
    let app = Router::new()
        .route("/", get(index))
        .nest("/api", api_router(state.clone()))
        .nest("/oauth", auth_router())
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

async fn index(State(state) : State<AppState>) -> Html<String> {
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

    //TODO: FOR TEST ONLY. REPLACE WITH ENV VARS WHEN AUTH 2.0 WILL END
    let is_admin_exists = is_account_already_exists_by_login("admin", &state).await;
    if !is_admin_exists {
        user_sign_up("admin", "qwerty123456", "qwerty123456", "BigBoss", "admin@test.ru", &state).await;
    }

    let is_client_already_exists = is_client_already_exists_by_client_name("api", &state).await;
    if !is_client_already_exists {
        create_client(generate_id().await.as_str(), "api", "qwerty", "http://localhost:8000/oauth_code_redirect", &state).await;
    }
    // END TODO

    // start threads

    let state_clone = state.clone();

    tokio::spawn(async move { run_background_tasks(&state_clone).await });
    run_server(&state).await;
}