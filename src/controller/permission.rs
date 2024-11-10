use chrono::{DateTime, Utc};
use derive_builder::WebApiGen;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use aide::axum::ApiRouter;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use axum_login::permission_required;

use diesel::{ PgConnection};
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use crate::api_auth::login_impl::AuthBackend;
use crate::api_doc::{default_resp_docs, empty_resp_docs};
use crate::db_models;
use crate::db_models::permission::Permission;

pub fn web_routes2(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
    let router_add = ApiRouter::new().api_route(
        "/create_entity",
        post_with( db_models::permission::web::create_entity, empty_resp_docs),
    );
    let router_read = ApiRouter::new()
        .api_route(
            "/get_entity_by_id/:id",
            get_with(db_models::permission::web::get_entity_by_id, default_resp_docs::<Permission>),
        )
        .api_route(
            "/get_entity_page",
            post_with(db_models::permission::web::get_entity_page, empty_resp_docs),
        );
    let router_update = ApiRouter::new().api_route(
        "/update_entity_by_id/:id",
        put_with(db_models::permission::web::update_entity_by_id, default_resp_docs::<Permission>),
    );
    let router_delete = ApiRouter::new().api_route(
        "/delete_entity_by_id/:id",
        delete_with(db_models::permission::web::delete_entity_by_id, default_resp_docs::<Permission>),
    );
    router_add
        .route_layer(permission_required!(AuthBackend, "users_add"))
        .merge(router_read.route_layer(permission_required!(AuthBackend, "users_read")))
        .merge(router_delete.route_layer(permission_required!(AuthBackend, "users_delete")))
        .merge(router_update.route_layer(permission_required!(AuthBackend, "users_update")))
        .with_state(conn_pool)
}
