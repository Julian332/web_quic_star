use crate::CONFIG;
use alloy::network::{EthereumWallet};

use alloy::providers::{Identity, Provider, ProviderBuilder, RootProvider};
use alloy::providers::fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller};

mod erc20;
pub mod uni_factory;
pub mod uni_pair;
pub mod uni_router2;
pub mod usdt;

pub fn get_project_signer() -> EthereumWallet {
    todo!()
}

pub fn signer_http_provider() -> impl Provider {
    ProviderBuilder::new()
        .wallet(get_project_signer())
        .connect_http(CONFIG.eth_rpc.clone())
}

pub fn http_provider() -> FillProvider<JoinFill<Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>>, RootProvider> {
    ProviderBuilder::new().connect_http(CONFIG.eth_rpc.clone())
}
