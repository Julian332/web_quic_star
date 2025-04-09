use crate::db_models::group::web::get_routers;
use crate::framework::auth::{AuthBackend, AuthPermission::*};
use aide::axum::ApiRouter;
use axum_login::permission_required;

pub fn group_router() -> ApiRouter {
    let (router_add, router_read, router_update, router_delete) = get_routers();
    router_add
        .route_layer(permission_required!(AuthBackend, Add("users")))
        .merge(router_read)
        .merge(router_delete.route_layer(permission_required!(AuthBackend, Delete("users"))))
        .merge(router_update.route_layer(permission_required!(AuthBackend, Update("users"))))
}
