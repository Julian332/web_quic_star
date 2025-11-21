#![allow(clippy::too_many_arguments)]
use crate::CONFIG;
use alloy::network::EthereumWallet;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
};

use alloy::providers::{Identity, Provider, ProviderBuilder, RootProvider};

mod erc20;
pub mod uni_factory;
pub mod uni_pair;
pub mod uni_router2;
pub mod usdt;

fn get_project_signer() -> EthereumWallet {
    todo!()
}

pub fn signer_http_provider() -> impl Provider {
    ProviderBuilder::new()
        .wallet(get_project_signer())
        .connect_http(CONFIG.eth_rpc.clone())
}
pub type ReadOnlyProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;
pub fn http_provider() -> ReadOnlyProvider {
    ProviderBuilder::new().connect_http(CONFIG.eth_rpc.clone())
}
