use alloy::sol;
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    USDT,
    "src/util/eth_contracts/abis/usdt.json"
);
