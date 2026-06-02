use crate::CONFIG;
use crate::framework::api_doc::{fallback, set_api_doc};
use crate::framework::auth::AuthPermission::Admin;
use crate::framework::auth::{AuthBackend, AuthPermission, get_auth_layer};
use crate::middleware::{global_req_state, log_req};
use aide::axum::ApiRouter;
use axum::Router;
use axum::middleware::from_fn;
use axum_login::require::{BoxFuture, Decision, DecisionPredicate, Require};
use axum_login::{AuthSession, AuthzBackend};
use http::{HeaderValue, Method};
use std::ops::Deref;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::services::ServeDir;

pub mod auth;
pub mod docs;
pub mod group;
pub mod upload;
pub mod user;

#[allow(clippy::unwrap_used)]
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
        // enable if needed
        .layer(from_fn(crate::middleware::save_req::save_req_to_db))
        // enable if needed
        .layer(from_fn(crate::middleware::continue_when_drop_req))
        .layer(from_fn(log_req))
        .layer(from_fn(global_req_state))
        .layer(get_auth_layer())
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
        //10MB
        .layer(RequestBodyLimitLayer::new(102400));
    if CONFIG.is_dev {
        let server_port = CONFIG.server_port;
        tracing::info!("swagger docs are accessible at http://127.0.0.1:{server_port}/docs");
        tracing::info!(
            "pretty docs are accessible at http://127.0.0.1:{server_port}/docs/pretty_doc"
        );
    }

    set_api_doc(app)
}
struct PermissionsPredicateWithAdmin {
    perms: Arc<Vec<AuthPermission>>,
    require_all: bool,
}

impl DecisionPredicate<AuthBackend> for PermissionsPredicateWithAdmin {
    fn decide(
        &self,
        auth_session: AuthSession<AuthBackend>,
        _state: Arc<()>,
    ) -> BoxFuture<'static, Decision> {
        let required_permissions = Arc::clone(&self.perms);
        let require_all = self.require_all;
        Box::pin(async move {
            let Some(user) = auth_session.user().await else {
                return Decision::Unauthenticated;
            };

            match auth_session.backend().get_all_permissions(&user).await {
                Err(_) => Decision::Unauthorized,
                Ok(perms) => {
                    if perms.contains(&Admin) {
                        return Decision::Allow;
                    };
                    let allow = if require_all {
                        required_permissions.iter().all(|x| perms.contains(x))
                    } else {
                        required_permissions.iter().any(|x| perms.contains(x))
                    };
                    if allow {
                        Decision::Allow
                    } else {
                        Decision::Unauthorized
                    }
                }
            }
        })
    }
}
pub fn require_permissions<Perms: IntoIterator<Item = AuthPermission<&'static str>>>(
    perms: Perms,
) -> Require<AuthBackend> {
    let predicate_with_admin = PermissionsPredicateWithAdmin {
        perms: Arc::new(perms.into_iter().map(|x| x.into()).collect()),
        require_all: true,
    };

    Require::<AuthBackend>::builder()
        .decision(predicate_with_admin)
        .build()
}
pub fn require_login() -> Require<AuthBackend> {
    Require::<AuthBackend>::builder().build()
}
