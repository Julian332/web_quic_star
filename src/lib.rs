#![forbid(unsafe_code)]
use crate::config::Config;
use crate::db_models::ConnPool;
use crate::framework::db::setup_connection_pool;
use framework::errors::AppError;
use std::sync::LazyLock;

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
pub static DB: LazyLock<ConnPool> = LazyLock::new(|| setup_connection_pool());
pub static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());
#[cfg(feature = "solana_mode")]
pub static SOL_CLIENT: LazyLock<anchor_client::solana_client::nonblocking::rpc_client::RpcClient> =
    LazyLock::new(|| {
        anchor_client::solana_client::nonblocking::rpc_client::RpcClient::new(
            CONFIG.solana_rpc.to_string(),
        )
    });
#[cfg(feature = "eth_mode")]
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
};
#[cfg(feature = "eth_mode")]
use alloy::providers::{Identity, RootProvider};
#[cfg(feature = "eth_mode")]
pub static ETH_CLIENT: LazyLock<
    FillProvider<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        RootProvider,
    >,
> = LazyLock::new(utils::contracts::http_provider);

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    config::set_env();
    envy::from_env().unwrap()
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
