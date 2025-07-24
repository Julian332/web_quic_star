use std::env;
use web_quick::framework::api_doc::set_api_doc;
use web_quick::scheduled_task::set_scheduler;
use web_quick::{api_router, set_env, set_log};

#[tokio::main]
async fn main() {
    set_log();
    set_env();
    set_scheduler().await;

    aide::generate::extract_schemas(true);

    let app = api_router::setup_router();

    let doc_app = set_api_doc(app);
    let server_port = env::var("SERVER_PORT").unwrap_or("5090".to_string());
    #[cfg(feature = "dev")]
    tracing::info!("swagger docs are accessible at http://127.0.0.1:{server_port}/docs");
    #[cfg(feature = "dev")]
    tracing::info!("pretty docs are accessible at http://127.0.0.1:{server_port}/docs/pretty_doc");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{server_port}"))
        .await
        .expect("Can not bind to port");
    axum::serve(listener, doc_app)
        .await
        .expect("Can not run server");
}
