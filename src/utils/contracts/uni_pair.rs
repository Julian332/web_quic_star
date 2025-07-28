use crate::utils::contracts::uni_factory::{uni_factory_addr, UNI_FACTORY};
use alloy::primitives::Address;
use crate::{AppRes, ETH_CLIENT};
use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UNI_PAIR,
    "src/utils/contracts/abis/uni_pair.json"
);

pub async fn get_pair(token_a: Address, token_b: Address) -> AppRes<Address> {
    let uni_factory = UNI_FACTORY::new(uni_factory_addr().await?, ETH_CLIENT.clone());
    Ok(uni_factory.getPair(token_a, token_b).call().await?)
}
