use std::env;

use aide::axum::ApiRouter;
use http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use web_quick::db_models::ConnPool;
use web_quick::framework::api_doc::{fallback, set_api_doc};
use web_quick::framework::auth::get_auth_layer;
use web_quick::framework::db::setup_connection_pool;
use web_quick::scheduled_task::set_scheduler;
use web_quick::{api, set_env, set_log, FILE_SERVER_DIRECTORY};

#[tokio::main]
async fn main() {
    set_log();
    set_env();

    let connection_pool = setup_connection_pool();
    set_scheduler(connection_pool.clone()).await;

    aide::generate::extract_schemas(true);

    let app = api::setup_router(connection_pool);

    let doc_app = set_api_doc(app);
    let server_port = env::var("SERVER_PORT").unwrap_or("5090".to_string());
    #[cfg(feature = "dev")]
    tracing::info!("Api docs are accessible at http://127.0.0.1:{server_port}/docs");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{server_port}"))
        .await
        .expect("Can not bind to port");
    axum::serve(listener, doc_app)
        .await
        .expect("Can not run server");
}
