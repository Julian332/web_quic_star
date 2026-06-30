use web3_quick::framework::db;

#[tokio::main]
async fn main() {
    db::sync_db_schema().await;
}
