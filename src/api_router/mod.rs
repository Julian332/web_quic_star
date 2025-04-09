use crate::config::FILE_SERVER_DIRECTORY;
use crate::framework::api_doc::fallback;
use crate::framework::auth::get_auth_layer;
use aide::axum::ApiRouter;
use http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub mod auth;
pub mod docs;
pub mod group;
pub mod upload;
pub mod user;

pub fn setup_router() -> ApiRouter {
    let app = ApiRouter::new()
        .nest_api_service("/auth", crate::api_router::auth::router())
        .nest_api_service("/users", crate::api_router::user::user_routes())
        .nest_api_service(
            "/user_with_group",
            crate::db_models::user_with_group_views::web_routes(),
        )
        .nest_api_service("/groups", crate::api_router::group::group_router())
        .nest_api_service("/upload", crate::api_router::upload::upload_routes())
        .nest_service(FILE_SERVER_DIRECTORY, ServeDir::new("assets"))
        .fallback(fallback)
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().expect("allow_origin is wrong"))
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
        .layer(get_auth_layer());
    app
}
