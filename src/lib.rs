#![forbid(unsafe_code)]
extern crate core;

use crate::db_models::ConnPool;
use crate::framework::db::setup_connection_pool;
use framework::errors::AppError;
use std::env;
use std::sync::{LazyLock};
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
pub mod third_party_api;
pub mod utils;
// todo Progress bar
// todo without native db driver
pub type AppRes<T> = Result<T, AppError>;
pub static DB: LazyLock<ConnPool> = LazyLock::new(|| {
    config::set_log();
    config::set_env();
    setup_connection_pool()
});
pub static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());
#[cfg(feature = "solana_mode")]
pub static SOL_RPC: LazyLock<anchor_client::solana_client::nonblocking::rpc_client::RpcClient> =
    LazyLock::new(|| {
        anchor_client::solana_client::nonblocking::rpc_client::RpcClient::new(
            env::var("SOLANA_RPC").expect("SOLANA_RPC must be set"),
        )
    });

#[macro_export]
macro_rules! unwrap_opt_or_continue {
    ($res:expr) => {
        match $res {
            None => {
                tracing::warn!("none value, skipped");
                continue;
            }
            Some(val) => val,
        }
    };
}
#[macro_export]
macro_rules! unwrap_or_continue {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                tracing::warn!("An error skipped: {};", e);
                continue;
            }
        }
    };
}
