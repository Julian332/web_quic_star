use crate::db_models::permission;
use crate::framework::auth::AuthBackend;
use aide::axum::ApiRouter;
use axum_login::permission_required;

pub fn permission_routes() -> ApiRouter {
    let (router_add, router_read, router_update, router_delete) = permission::web::get_routers();

    router_add
        .route_layer(permission_required!(AuthBackend, "users_add"))
        .merge(router_read.route_layer(permission_required!(AuthBackend, "users_read")))
        .merge(router_delete.route_layer(permission_required!(AuthBackend, "users_delete")))
        .merge(router_update.route_layer(permission_required!(AuthBackend, "users_update")))
}
