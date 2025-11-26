#![forbid(unsafe_code)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::indexing_slicing)]
#![allow(clippy::get_first)]
extern crate core;
use crate::config::Config;
use crate::framework::db::setup_connection_pool;
use framework::db::ConnPool;
use framework::errors::AppError;
use std::sync::LazyLock;

pub mod api_router;
pub mod api_service;
pub mod api_wrapper;
pub mod config;
pub mod db_model;
pub mod domain;
pub mod framework;
pub mod middleware;
pub mod scheduled_task;
pub mod schema;
pub mod schema_view;
#[cfg(feature = "solana_mode")]
pub mod subscribe;
pub mod util;

pub mod prelude {
    pub use super::util::tokio::*;
    pub use chrono::prelude::*;
    pub use diesel::prelude::*;
    pub use rust_decimal::prelude::*;

    pub use super::CONFIG;
    pub use super::DB;

    pub use super::framework::db::{LogicDeleteQuery, Paginate};
    pub use super::framework::errors::IntoResult;
    pub use super::util::datetime::{TimeUtil, chinese_datetime_format};
    pub use super::util::num_fmt::NumFmt;
    pub use diesel_async::RunQueryDsl;

    pub use super::AppRes;
    #[cfg(feature = "eth_mode")]
    pub use super::domain::eth_addr::EthAddr;
    #[cfg(feature = "solana_mode")]
    pub use super::domain::solana_addr::SolAddr;
    pub use super::framework::api::PageRes;
    pub use super::framework::errors::AppError;

    pub use super::unwrap_opt_or_continue;
    pub use super::unwrap_or_continue;
    pub use tracing::{debug, error, info, trace, warn};
}

// todo Progress bar
// todo slow sql , log sql

pub type AppRes<T> = Result<T, AppError>;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
task_local! {
    pub static CURRENT_REQ : ReqState ;
}
pub static DB: LazyLock<ConnPool> = LazyLock::new(setup_connection_pool);
pub static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);
#[cfg(feature = "solana_mode")]
pub static SOL_CLIENT: LazyLock<solana_client::nonblocking::rpc_client::RpcClient> =
    LazyLock::new(|| {
        solana_client::nonblocking::rpc_client::RpcClient::new(CONFIG.solana_rpc.to_string())
    });
use middleware::ReqState;

use mimalloc::MiMalloc;
use tokio::task_local;

#[cfg(feature = "eth_mode")]
pub static ETH_CLIENT: LazyLock<util::eth_contracts::ReadOnlyProvider> =
    LazyLock::new(util::eth_contracts::http_provider);

#[allow(clippy::expect_used)]
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    config::set_env();
    config::set_log();
    envy::from_env().expect(".env error")
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
