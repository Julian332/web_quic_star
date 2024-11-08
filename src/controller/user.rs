use crate::api_doc::errors::AppError;
use crate::api_doc::extractors::Json;
use crate::AppRes;
use aide::OperationIo;
use axum::extract::State;
use axum_login::{login_required, AuthSession};
use chrono::{DateTime, Utc};
use derive_builder::WebApiGen;
use diesel::{AsChangeset, Insertable, Queryable, RunQueryDsl, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Clone,
    Serialize,
    Deserialize,
    Selectable,
    JsonSchema,
    Default,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub group_id: i64,
    pub tenantry: String,
    pub remark: Option<String>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub is_delete: bool,
}
#[derive(
    Queryable,
    Clone,
    Serialize,
    Deserialize,
    Selectable,
    JsonSchema,
    Default,
    AsChangeset,
    Debug,
    WebApiGen,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub group_id: i64,
    pub tenantry: String,
    pub remark: Option<String>,
    pub update_time: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub update_by: Option<i64>,
    pub is_delete: bool,
}
pub fn web_routes2(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
    let router_add = ApiRouter::new().api_route(
        "/create_entity",
        post_with(web::create_entity, empty_resp_docs),
    );
    let router_read = ApiRouter::new()
        .api_route(
            "/get_entity_by_id/:id",
            get_with(web::get_entity_by_id, default_resp_docs::<User>),
        )
        .api_route(
            "/get_entity_page",
            post_with(web::get_entity_page, empty_resp_docs),
        );
    let router_update = ApiRouter::new().api_route(
        "/update_entity_by_id/:id",
        put_with(web::update_entity_by_id, default_resp_docs::<User>),
    );
    let router_delete = ApiRouter::new().api_route(
        "/delete_entity_by_id/:id",
        delete_with(web::delete_entity_by_id, default_resp_docs::<User>),
    );
    let modify_password = ApiRouter::new().api_route(
        "/modify_password",
        post_with(modify_password, default_resp_docs::<String>),
    );
    router_add
        .route_layer(permission_required!(AuthBackend, "users_add"))
        .merge(router_read.route_layer(permission_required!(AuthBackend, "users_read")))
        .merge(router_delete.route_layer(permission_required!(AuthBackend, "users_delete")))
        .merge(router_update.route_layer(permission_required!(AuthBackend, "users_update")))
        .merge(modify_password.route_layer(login_required!(AuthBackend)))
        .with_state(conn_pool)
}

#[derive(Serialize, Deserialize, OperationIo, Debug, Default, JsonSchema)]
struct ModifyPassword {
    old_password: String,
    new_password: String,
}
async fn modify_password(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    auth_session: AuthSession<AuthBackend>,
    Json(modify_password): Json<ModifyPassword>,
) -> AppRes<String> {
    if modify_password.new_password.len() < 8 {
        return Err(AppError::new(
            "password should be longer than or equal 8".to_string(),
        ));
    }
    match auth_session.user {
        None => return Err(AppError::new("not be".to_string())),
        Some(mut user) => {
            password_auth::verify_password(modify_password.old_password, &user.password)?;
            let hash = password_auth::generate_hash(modify_password.new_password);
            user.password = hash;
            diesel::update(users).set(user).execute(&mut pool.get()?)?;
        }
    }

    Ok("succeed".to_string())
}
