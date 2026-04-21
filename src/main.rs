use web3_quick::framework::db;
use web3_quick::scheduled_task::set_scheduler;
use web3_quick::{CONFIG, api_router};

#[tokio::main]
async fn main() {
    db::sync_db_schema().await;
    set_scheduler().await;
    let doc_app = api_router::setup_router();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", CONFIG.server_port))
        .await
        .expect("Can not bind to port");
    axum::serve(listener, doc_app)
        .await
        .expect("Can not run server");
}
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use anchor_client::solana_sdk::pubkey::Pubkey;
    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn async_test1() {}
    #[test]
    fn sync_test1() {
        let pubkey = Pubkey::from([
            6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218, 196, 57,
            220, 26, 235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
        ]);
        println!("{}", pubkey);
    }
}
