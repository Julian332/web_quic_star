use crate::framework::errors::AppError;
use crate::{AppRes, DB};
use aide::OperationIo;
use axum::Json;
use axum_login::{AuthSession, login_required};
use diesel::{QueryDsl, RunQueryDsl};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use axum_login::permission_required;

use crate::db_model::user;
use crate::framework::api_doc::default_resp_docs;
use crate::framework::auth::AuthBackend;
use crate::framework::auth::AuthPermission::*;
use crate::schema::users::dsl::users;

#[derive(Serialize, Deserialize, OperationIo, Debug, Default, JsonSchema)]
pub struct ModifyPassword {
    old_password: String,
    new_password: String,
}
pub(crate) async fn modify_password(
    auth_session: AuthSession<AuthBackend>,
    Json(modify_password): Json<ModifyPassword>,
) -> AppRes<String> {
    if modify_password.new_password.len() < 8 {
        return Err(AppError::new("password should be longer than or equal 8"));
    }
    match auth_session.user().await {
        None => return Err(AppError::new("not be")),
        Some(mut user) => {
            password_auth::verify_password(modify_password.old_password, &user.password)?;
            let hash = password_auth::generate_hash(modify_password.new_password);
            user.password = hash;
            diesel::update(users.find(user.id))
                .set(user)
                .execute(&mut DB.get()?)?;
        }
    }

    Ok("succeed".to_string())
}

pub fn user_routes() -> ApiRouter {
    let (router_add, router_read, router_update, router_delete) = user::web::get_routers();
    let modify_password = ApiRouter::new().api_route(
        "/modify_password",
        post_with(modify_password, default_resp_docs::<String>),
    );
    router_add
        .route_layer(permission_required!(AuthBackend, Add("users")))
        .merge(router_read.route_layer(permission_required!(AuthBackend, Read("users"))))
        .merge(router_delete.route_layer(permission_required!(AuthBackend, Delete("users"))))
        .merge(router_update.route_layer(permission_required!(AuthBackend, Update("users"))))
        .merge(modify_password.route_layer(login_required!(AuthBackend)))
}
