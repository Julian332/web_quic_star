use alloy::sol;
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    USDT,
    "src/util/contracts/abis/usdt.json"
);
