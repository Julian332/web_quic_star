use alloy::sol;
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UNI_PAIR,
    "src/util/eth_contracts/abis/uni_pair.json"
);
