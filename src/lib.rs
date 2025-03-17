#![forbid(unsafe_code)]
use crate::db_models::ConnPool;
use crate::framework::db::setup_connection_pool;
use derive_more::{Display, Error};
use framework::errors::AppError;
use std::collections::HashMap;
use std::panic;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;
use tracing::error;
use tracing_subscriber::EnvFilter;

pub mod api_router;
pub mod config;
pub mod db_models;
pub mod domain;
pub mod framework;
pub mod scheduled_task;
pub mod schema;
pub mod schema_view;
#[cfg(feature = "solana_mode")]
pub mod subscribe;
pub mod utils;

//todo global soft delete ,toggle by feature
//todo global multi TENANTRY ,toggle by feature
type AppRes<T> = Result<T, AppError>;
pub type Cache<K, V> = LazyLock<Arc<RwLock<HashMap<K, V>>>>;
pub static DB: LazyLock<ConnPool> = LazyLock::new(|| setup_connection_pool());
pub static HTTP: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

#[allow(unused)]
#[derive(Debug, Display, Error)]
pub struct NoneError;
pub fn set_env() {
    #[cfg(feature = "dev")]
    {
        set_dev_env()
    }
    #[cfg(not(feature = "dev"))]
    {
        set_prod_env()
    }
}

pub fn set_dev_env() {
    tracing::info!("profile :{} is active", "dev");
    dotenvy::from_filename(".env").expect("no .env file");
}

pub fn set_prod_env() {
    tracing::info!("profile :{} is active", "release");
    dotenvy::from_filename("env_prod.env").expect("no env_prod.env file");
}
pub fn set_log() {
    panic::set_hook(Box::new(|info| {
        error!(error = %info, "panic occurred");
    }));
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .event_format(
            tracing_subscriber::fmt::format()
                // .with_file(true)
                .with_line_number(true),
        )
        .init();
    aide::generate::on_error(|error| {
        error!("{error}");
    });
}
#[test]
pub fn test() {
    set_log();
    use tracing::error;

    let (err_info, port) = ("No connection", 22);

    error!(err_info);
    error!(target: "app_events", "App Error: {}", err_info);
    error!({ info = err_info }, "error on port: {}", port);
    error!(name: "invalid_input", "Invalid input: {}", err_info);
}
