use crate::utils::contracts::uni_router2::{ UNI_ROUTER2};
use crate::{AppRes, CONFIG, ETH_CLIENT};
use alloy::primitives::Address;
use alloy::sol;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UNI_FACTORY,
    "src/utils/contracts/abis/uni_factory.json"
);

pub async fn uni_factory_addr() -> AppRes<Address> {
    let uni_router2 = UNI_ROUTER2::new(CONFIG.eth_addrs.uni_router2_addr.0, ETH_CLIENT.clone());
    Ok(uni_router2.factory().call().await?)
}
