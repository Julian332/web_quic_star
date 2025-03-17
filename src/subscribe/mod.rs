use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::Cluster;
use std::env;

pub fn get_admin_keypair() -> Keypair {
    let result = env::var("ADMIN_PK").expect("ADMIN_PK environment variable not specified");
    Keypair::from_base58_string(&result)
}

pub fn get_cluster() -> Cluster {
    let string = get_rpc_uri();
    let ws_string =
        env::var("SOLANA_WS_RPC").expect("SOLANA_WS_RPC environment variable not specified");
    Cluster::Custom(string, ws_string)
}
pub fn get_rpc_uri() -> String {
    env::var("SOLANA_RPC").expect("SOLANA_RPC environment variable not specified")
}
pub fn get_wss_uri() -> String {
    env::var("SOLANA_WS_RPC").expect("SOLANA_WS_RPC environment variable not specified")
}

use crate::db_models::ConnPool;
use crate::AppRes;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;

pub async fn subscribe_with_retry<F>(pool: ConnPool, func: fn(ConnPool) -> F) -> AppRes<()>
where
    F: Future<Output = AppRes<()>> + Sized,
{
    loop {
        match func(pool.clone()).await {
            Ok(_) => {}
            Err(e) => {
                error!(?e, " Will retry subscribe");
                sleep(Duration::new(2, 0)).await;
            }
        };
    }
}
