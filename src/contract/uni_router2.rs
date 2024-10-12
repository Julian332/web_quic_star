//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

// use alloy::{node_bindings::Anvil, primitives::address, providers::ProviderBuilder, sol};
// use eyre::Result;

use std::env;
use std::error::Error;
use std::str::FromStr;

use alloy::network::{Ethereum, EthereumWallet};
use alloy::primitives::Address;
use alloy::providers::fillers::{FillProvider, JoinFill, RecommendedFiller, WalletFiller};
use alloy::providers::ReqwestProvider;
use alloy::sol;
use alloy::transports::http::{Client, Http};

use crate::contract::uni_router2::UNI_ROUTER2::UNI_ROUTER2Instance;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UNI_ROUTER2,
    "src/contract/uni_router2.json"
);



pub fn uni_router2_addr() -> Address {
    Address::from_str(env::var("UNI_ROUTER2_ADDR").expect(".env UNI_ROUTER2_ADDR").as_str()).expect(".env UNI_ROUTER2_ADDR")
}
