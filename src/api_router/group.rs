use crate::api_router::require_permissions;
use crate::db_model::group::web::get_routers;
use crate::framework::auth::AuthPermission::*;
use aide::axum::ApiRouter;

pub fn group_router() -> ApiRouter {
    let (router_add, router_read, router_update, router_delete) = get_routers();
    router_add
        .route_layer(require_permissions([Add("users")]))
        .merge(router_read)
        .merge(router_delete.route_layer(require_permissions([Delete("users")])))
        .merge(router_update.route_layer(require_permissions([Update("users")])))
}
