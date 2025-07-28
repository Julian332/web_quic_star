use crate::framework::api_doc::{fallback, set_api_doc};
use crate::framework::auth::get_auth_layer;
use aide::axum::ApiRouter;
use http::{HeaderValue, Method};
use std::ops::Deref;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use crate::CONFIG;

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
            crate::db_models::user_with_group_views::web_routes(),
        )
        .nest_api_service("/groups", group::group_router())
        .nest_api_service("/upload", upload::upload_routes())
        .nest_service(
            &format!("/{}", CONFIG.file_server_directory.deref()),
            ServeDir::new(CONFIG.file_server_directory.as_str()),
        )
        .fallback(fallback)
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().expect("allow_origin is wrong"))
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
        .layer(get_auth_layer());

    set_api_doc(app)
}
