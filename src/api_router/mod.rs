use crate::CONFIG;
use crate::framework::api_doc::{fallback, set_api_doc};
use crate::framework::auth::get_auth_layer;
use crate::web_middleware::{req_log, save_req_to_task_local};
use aide::axum::ApiRouter;
use axum::Router;
use axum::middleware::from_fn;
use http::{HeaderValue, Method};
use std::ops::Deref;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::services::ServeDir;

pub mod auth;
pub mod docs;
pub mod group;
pub mod upload;
pub mod user;

pub fn setup_router() -> Router {
    aide::generate::extract_schemas(true);

    let app = ApiRouter::new()
        .nest_api_service("/auth", auth::router())
        .nest_api_service("/users", user::user_routes())
        .nest_api_service(
            "/user_with_group",
            crate::db_model::user_with_group_views::web_routes(),
        )
        .nest_api_service("/groups", group::group_router())
        .nest_api_service("/upload", upload::upload_routes())
        .nest_service(
            &format!("/{}", CONFIG.file_server_directory.deref()),
            ServeDir::new(CONFIG.file_server_directory.as_str()),
        )
        .fallback(fallback)
        .layer(from_fn(req_log))
        .layer(from_fn(save_req_to_task_local))
        .layer(get_auth_layer())
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
        //10MB
        .layer(RequestBodyLimitLayer::new(102400));
    #[cfg(feature = "dev")]
    {
        let server_port = CONFIG.server_port;
        tracing::info!("swagger docs are accessible at http://127.0.0.1:{server_port}/docs");
        tracing::info!(
            "pretty docs are accessible at http://127.0.0.1:{server_port}/docs/pretty_doc"
        );
    }

    set_api_doc(app)
}
