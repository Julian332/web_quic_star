use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UniGraphParams {
    #[serde(rename = "variables")]
    pub variables: Variables,

    #[serde(rename = "query")]
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variables {
    #[serde(rename = "addr")]
    pub addr: String,

    #[serde(rename = "timestamp")]
    pub timestamp: i64,
}
