//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

// use alloy::{node_bindings::Anvil, primitives::address, providers::ProviderBuilder, sol};
// use eyre::Result;

use crate::utils::contracts::readonly_ws_provider;
use crate::utils::contracts::uni_router2::{uni_router2_addr, UNI_ROUTER2};
use crate::AppRes;
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
    let uni_router2 = UNI_ROUTER2::new(uni_router2_addr(), readonly_ws_provider());
    Ok(uni_router2.factory().call().await?._0)
}
