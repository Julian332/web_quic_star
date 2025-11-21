use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC_20,
    "src/util/eth_contracts/abis/erc20.json"
);
