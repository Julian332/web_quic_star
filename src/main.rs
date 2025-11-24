use web3_quick::framework::db;
use web3_quick::scheduled_task::set_scheduler;
use web3_quick::{CONFIG, api_router, config};

#[tokio::main]
async fn main() {
    config::set_log();
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
