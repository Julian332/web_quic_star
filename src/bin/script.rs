use redis::AsyncTypedCommands;
use serde_json::Value::Array;
use serde_json::{Value, json};
use web3_quick::REDIS;
#[tokio::main]
async fn main() {
    let resp = include_str!("resp.json");
    let resp: Value = serde_json::from_str(resp).unwrap();
    let Array(resp) = resp else {
        return;
    };
    let resp: Vec<_> = resp
        .iter()
        .filter_map(|x| {
            let option = x.as_object().unwrap().get("label").unwrap();
            let is_dev = option.as_str().unwrap().contains("Dev");
            if is_dev {
                let option1 = x.get("address").unwrap().as_str().unwrap();
                Some((option1, "true"))
            } else {
                None
            }
        })
        .collect();
    println!("{:?}", resp);
    let pin = REDIS
        .clone()
        .hset_multiple("devWalletMap:8453", resp.as_slice())
        .await
        .unwrap();
}
