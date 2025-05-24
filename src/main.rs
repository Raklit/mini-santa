mod core;

use crate::core::config::AppConfig;
use crate::core::data_model::traits::IAccount;

use axum:: {
    routing::get,
    Router,
    extract::State,
    response::Html
};

use sqlx::{Pool, SqlitePool};
use tracing_subscriber::{filter, prelude::*};
use uuid::Uuid;
use core::controllers::{healthcheck, sign_in};
use core::data_model::implementations::Account;
use core::data_model::traits::ILocalObject;
use core::functions::init_database;
use core::services::{create_account, create_client, get_account_by_login, is_account_already_exists_by_login, is_client_already_exists_by_client_name, set_account_password};
use std::fs::{self, OpenOptions};
use std::net::SocketAddr;
use tower_http::trace::{DefaultOnResponse, DefaultMakeSpan, TraceLayer};
use tracing::Level;
use tower_http::services::ServeDir;
use tera::{Tera, Context};
use sqlx::{migrate::MigrateDatabase, Sqlite, Row};

#[derive(Clone)]
struct AppState {
    tera : Tera,
    context : Context,
    db : SqlitePool
}

async fn index(State(state) : State<AppState>) -> Html<String> {
    let temp = match state.tera.render("index.html", &(state.context)) {
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
        tera: tera,
        context: context,
        db: db
    };

    init_database(&state).await;
    //TODO: FOR TEST ONLY. REPLACE WITH ENV VARS WHEN AUTH 2.0 WILL END
    let is_admin_exists = is_account_already_exists_by_login("admin", &state).await;
    if !is_admin_exists {
        create_account(Uuid::new_v4().to_string().as_str(),"admin", "qwerty", &state).await;
    }

    let is_client_already_exists = is_client_already_exists_by_client_name("api", &state).await;
    if !is_client_already_exists {
        create_client(Uuid::new_v4().to_string().as_str(), "api", "qwerty", &state).await;
    }

    let account = get_account_by_login("admin", &state).await.unwrap();
    tracing::info!("{}, {}, {}", account.id(), account.password_hash(), account.passwrod_salt());
    // END TODO

    let app = Router::new()
        .route("/", get(index))
        .route("/api/healthcheck", get(healthcheck))
        .route("/oauth/token", get(sign_in))
        .with_state(state)
        .nest_service("/static", ServeDir::new("./app/static"))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}