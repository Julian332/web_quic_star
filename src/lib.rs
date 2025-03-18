#![forbid(unsafe_code)]
use crate::db_models::ConnPool;
use crate::framework::db::setup_connection_pool;
use framework::errors::AppError;
use std::sync::{Arc, LazyLock};
use std::{env, panic};
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
pub type AppRes<T> = Result<T, AppError>;
pub static DB: LazyLock<ConnPool> = LazyLock::new(|| setup_connection_pool());
pub static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());
#[cfg(feature = "solana_mode")]
pub static SOL_RPC: LazyLock<anchor_client::solana_client::nonblocking::rpc_client::RpcClient> =
    LazyLock::new(|| {
        anchor_client::solana_client::nonblocking::rpc_client::RpcClient::new(
            env::var("SOLANA_RPC").expect("SOLANA_RPC must be set"),
        )
    });

#[cfg(feature = "solana_mode")]
pub static ANCHOR: LazyLock<
    anchor_client::Program<Arc<anchor_client::solana_sdk::signature::Keypair>>,
> = LazyLock::new(|| {
    anchor_client::Client::new(
        anchor_client::Cluster::Custom(
            env::var("SOLANA_RPC").expect("SOLANA_RPC must be set"),
            env::var("WS_SOLANA_RPC").expect("WS_SOLANA_RPC must be set"),
        ),
        Arc::new(anchor_client::solana_sdk::signature::Keypair::new()),
    )
    .program(anchor_client::solana_sdk::system_program::ID)
    .expect("init anchor failed")
});

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
