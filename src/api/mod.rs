use crate::db_models::ConnPool;
use crate::framework::api_doc::fallback;
use crate::framework::auth::get_auth_layer;
use crate::FILE_SERVER_DIRECTORY;
use aide::axum::ApiRouter;
use http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub mod auth;
pub mod group;
pub mod group_permission;
pub mod permission;
pub mod upload;
pub mod user;

pub fn setup_router(connection_pool: ConnPool) -> ApiRouter {
    let app = ApiRouter::new()
        .nest_api_service("/auth", crate::api::auth::router())
        .nest_api_service(
            "/users",
            crate::api::user::user_routes(connection_pool.clone()),
        )
        .nest_api_service(
            "/user_with_group",
            crate::db_models::user_with_group_views::web_routes(connection_pool.clone()),
        )
        .nest_api_service(
            "/groups",
            crate::api::group::group_router(connection_pool.clone()),
        )
        .nest_api_service("/upload", crate::api::upload::upload_routes())
        .nest_service(FILE_SERVER_DIRECTORY, ServeDir::new("assets"))
        .nest_api_service(
            "/permissions",
            crate::api::permission::permission_routes(connection_pool.clone()),
        )
        .nest_api_service(
            "/group_permission",
            crate::api::group_permission::group_permission_routes(connection_pool.clone()),
        )
        .fallback(fallback)
        .with_state(connection_pool.clone())
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
        .layer(get_auth_layer(connection_pool.clone()));
    app
}
