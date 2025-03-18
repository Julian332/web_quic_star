//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

// use alloy::{node_bindings::Anvil, primitives::address, providers::ProviderBuilder, sol};
// use eyre::Result;

use crate::utils::contracts::readonly_ws_provider;
use crate::utils::contracts::uni_factory::{uni_factory_addr, UNI_FACTORY};

use alloy::primitives::Address;

use crate::AppRes;
use alloy::sol;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UNI_PAIR,
    "src/utils/contracts/abis/uni_pair.json"
);

pub async fn get_pair(token_a: Address, token_b: Address) -> AppRes<Address> {
    let uni_factory = UNI_FACTORY::new(uni_factory_addr().await?, readonly_ws_provider());
    Ok(uni_factory.getPair(token_a, token_b).call().await?._0)
}
