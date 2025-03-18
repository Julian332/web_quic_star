use crate::utils::contracts::uni_pair::{get_pair, UNI_PAIR};
use crate::utils::contracts::usdt::usdt_addr;
use crate::AppRes;
use alloy::network::{Ethereum, EthereumWallet, TransactionBuilder};
use alloy::primitives::aliases::U112;
use alloy::primitives::{Address, TxHash, U256};
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
    RecommendedFiller, WalletFiller,
};
use alloy::providers::{
    Identity, Provider, ProviderBuilder, ReqwestProvider, RootProvider, WsConnect,
};
use alloy::pubsub::PubSubFrontend;
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::{Client, Http};
use reqwest::Url;
use std::env;
use std::error::Error;
use std::str::FromStr;

mod erc20;
pub mod uni_factory;
pub mod uni_pair;
pub mod uni_router2;
mod usdt;

// pub fn readonly_http_provider() -> FillProvider<
//     JoinFill<
//         Identity,
//         JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
//     >,
//     ReqwestProvider,
//     Http<Client>,
//     Ethereum,
// > {
//     ProviderBuilder::new()
//         .on_http(Url::from_str(env::var("ETH_RPC").expect(".env ETH_RPC not set").as_str()).expect())
// }

pub async fn readonly_ws_provider() -> AppRes<RootProvider<PubSubFrontend>> {
    let rpc_url = env::var("WS_ETH_RPC")?;
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;
    Ok(provider)
}

pub fn weth_addr() -> Address {
    Address::from_str(env::var("WETH_ADDR").expect(".env WETH_ADDR").as_str())
        .expect(".env WETH_ADDR")
}

// pub async fn get_dollar_price<T: Into<U112>>(x: T) -> AppRes<u128> {
//     let usdt_pair = UNI_PAIR::new(
//         get_pair(usdt_addr(), weth_addr()).await?,
//         readonly_ws_provider(),
//     );
//     let reserves_return = usdt_pair.getReserves().call().await?;
//
//     let token0 = usdt_pair.token0().call().await?._0;
//     Ok(if token0 == weth_addr() {
//         reserves_return._reserve1 * x.into() / reserves_return._reserve0
//     } else {
//         reserves_return._reserve0 * x.into() / reserves_return._reserve1
//     }
//     .to::<u128>())
// }
pub fn get_project_signer() -> EthereumWallet {
    let project_pk = env::var("PROJECT_SIGNER").expect(".env PROJECT_SIGNER");
    let signer = PrivateKeySigner::from_str(&project_pk).expect(".env PROJECT_SIGNER");
    EthereumWallet::new(signer)
}

// pub fn signer_http_provider() -> FillProvider<
//     JoinFill<
//         JoinFill<
//             Identity,
//             JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
//         >,
//         WalletFiller<EthereumWallet>,
//     >,
//     ReqwestProvider,
//     Http<Client>,
//     Ethereum,
// > {
//     ProviderBuilder::new()
//         .with_recommended_fillers()
//         .wallet(get_project_signer())
//         .on_http(eth_http_rpc())
// }

pub fn eth_http_rpc() -> Url {
    Url::from_str(env::var("ETH_RPC").expect(".env ETH_RPC").as_str()).unwrap()
}
