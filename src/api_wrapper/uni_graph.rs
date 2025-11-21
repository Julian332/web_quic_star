use crate::CONFIG;
use crate::prelude::IntoResult;
use chrono::Days;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct UniGraphResp {
    #[serde(rename = "data")]
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "swaps")]
    swaps: Vec<Swap>,
}

#[derive(Serialize, Deserialize)]
pub struct Swap {
    #[serde(rename = "amount0In")]
    pub amount0_in: String,

    #[serde(rename = "amount1In")]
    pub amount1_in: String,

    #[serde(rename = "amount1Out")]
    pub amount1_out: String,

    #[serde(rename = "amount0Out")]
    pub amount0_out: String,

    #[serde(rename = "from")]
    pub from: String,

    #[serde(rename = "pair")]
    pub pair: Pair,

    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct Pair {
    #[serde(rename = "token0")]
    pub token0: Token,

    #[serde(rename = "token1")]
    pub token1: Token,

    #[serde(rename = "token0Price")]
    pub token0_price: String,

    #[serde(rename = "token1Price")]
    pub token1_price: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "id")]
    pub id: String,
}

async fn _graph() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("accept", "application/json, multipart/mixed".parse()?);
    headers.insert(
        "accept-language",
        "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6".parse()?,
    );
    headers.insert("content-type", "application/json".parse()?);
    headers.insert("dnt", "1".parse()?);
    headers.insert("origin", "https://subgraph.satsuma-prod.com".parse()?);
    headers.insert("priority", "u=1, i".parse()?);
    headers.insert(
        "referer",
        "https://subgraph.satsuma-prod.com/julians-team--193144/univ2/playground".parse()?,
    );
    headers.insert(
        "sec-ch-ua",
        "\"Not)A;Brand\";v=\"99\", \"Microsoft Edge\";v=\"127\", \"Chromium\";v=\"127\"".parse()?,
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse()?);
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse()?);
    headers.insert("sec-fetch-dest", "empty".parse()?);
    headers.insert("sec-fetch-mode", "cors".parse()?);
    headers.insert("sec-fetch-site", "same-origin".parse()?);
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0".parse()?);

    let data = r#"
{
    "query": "  query getShop($addr: String!,$timestamp:BigInt!){\n  swaps(\n    where: {and: [{from: $addr }\n    , {or: [{pair_: {token0: \"0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2\"}}, {pair_: {token1: \"0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2\"}}]}\n    ,{timestamp_gte: $timestamp }]\n    \n    }\n  ) {\n    from\n    amount0In\n    amount0Out\n    amount1In\n    amount1Out\n    \n    \n    timestamp\n    pair {\n        token0Price\n        token1Price\n      token0 {\n        id\n      }\n      token1 {\n        id\n      }\n    }\n  }\n}",
    "variables": {
  "addr": "0x4d521577f820525964c392352bb220482f1aa63b",
  "timestamp": 1722995377
}
}
"#;
    let json: serde_json::Value = serde_json::from_str(data)?;

    let request = client
        .request(
            reqwest::Method::POST,
            "https://subgraph.satsuma-prod.com/7e1fb37825fa/julians-team--193144/univ2/api",
        )
        .headers(headers)
        .json(&json);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

pub async fn get_user_swaps(user_addr: String) -> Result<Vec<Swap>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("accept", "application/json, multipart/mixed".parse()?);
    headers.insert(
        "accept-language",
        "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6".parse()?,
    );
    headers.insert("content-type", "application/json".parse()?);
    headers.insert("dnt", "1".parse()?);
    headers.insert("origin", "https://subgraph.satsuma-prod.com".parse()?);
    headers.insert("priority", "u=1, i".parse()?);
    headers.insert(
        "referer",
        "https://subgraph.satsuma-prod.com/julians-team--193144/univ2/playground".parse()?,
    );
    headers.insert(
        "sec-ch-ua",
        "\"Not)A;Brand\";v=\"99\", \"Microsoft Edge\";v=\"127\", \"Chromium\";v=\"127\"".parse()?,
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse()?);
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse()?);
    headers.insert("sec-fetch-dest", "empty".parse()?);
    headers.insert("sec-fetch-mode", "cors".parse()?);
    headers.insert("sec-fetch-site", "same-origin".parse()?);
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0".parse()?);

    let three_day_ago = chrono::Utc::now()
        .checked_sub_days(Days::new(3))
        .into_result()?
        .timestamp();
    let data = r#"
{
    "query": "  query getShop($addr: String!,$timestamp:BigInt!){\n  swaps(\n    where: {and: [{from: $addr }\n    , {or: [{pair_: {token0: \"0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2\"}}, {pair_: {token1: \"0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2\"}}]}\n    ,{timestamp_gte: $timestamp }]\n    \n    }\n  ) {\n    from\n    amount0In\n    amount0Out\n    amount1In\n    amount1Out\n    \n    \n    timestamp\n    pair {\n        token0Price\n        token1Price\n      token0 {\n        id\n      }\n      token1 {\n        id\n      }\n    }\n  }\n}",
    "variables": {
      "addr": "0x4d521577f820525964c392352bb220482f1aa63b",
      "timestamp": 1722995377
}
}
"#;

    let mut json: Value = serde_json::from_str(data)?;
    let variables = json.get_mut("variables").into_result()?;
    let addr = variables.get_mut("addr").into_result()?;
    *addr = Value::from(user_addr);

    let timestamp = variables.get_mut("timestamp").into_result()?;
    *timestamp = Value::from(three_day_ago);
    let request = client
        .request(reqwest::Method::POST, CONFIG.uni_graph_url.clone())
        .headers(headers)
        .json(&json);

    let response = request.send().await?;
    let body = response.text().await?;
    // println!("{}", body);
    let uni_graph_resp = serde_json::from_str::<UniGraphResp>(&body)?;

    Ok(uni_graph_resp.data.swaps)
}
