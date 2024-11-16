// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use derive_builder::WebApiGen;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(clippy::all)]
#[derive(Deserialize, Serialize, JsonSchema, Default, Clone)]
pub struct GroupsPermissionBuilder {
    pub group_id: ::derive_builder::export::core::option::Option<Filter<i64>>,
    pub permission_id: ::derive_builder::export::core::option::Option<Filter<i64>>,
}
use crate::api_auth::login_impl::AuthBackend;
use crate::api_doc::{default_resp_docs, empty_resp_docs};
use crate::controller::Compare;
use crate::controller::Filter;
use crate::controller::LOGIN_URL;
use crate::db_models::group_permission::GroupsPermission;
use crate::db_models::ConnPool;
use crate::schema::groups_permissions::dsl::groups_permissions;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use aide::axum::ApiRouter;
use axum::extract::Path;
use axum_login::permission_required;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn web_routes(conn_pool: ConnPool) -> ApiRouter {
    let router_add = ApiRouter::new().api_route(
        "/create_entity",
        post_with(web::create_entity, empty_resp_docs),
    );
    let router_read = ApiRouter::new()
        .api_route(
            "/get_entity_by_id/:group_id/:permission_id",
            get_with(web::get_entity_by_id, default_resp_docs::<GroupsPermission>),
        )
        .api_route(
            "/get_entity_page",
            post_with(web::get_entity_page, empty_resp_docs),
        );
    let router_delete = ApiRouter::new().api_route(
        "/delete_entity_by_id/:group_id/:permission_id",
        delete_with(
            web::delete_entity_by_id,
            default_resp_docs::<GroupsPermission>,
        ),
    );
    router_add
        .route_layer(permission_required!(AuthBackend, "users_add"))
        .merge(router_read.route_layer(permission_required!(AuthBackend, "users_read")))
        .merge(router_delete.route_layer(permission_required!(AuthBackend, "users_delete")))
        .with_state(conn_pool)
}
mod web {
    use super::*;
    use crate::api_doc::errors::AppError;
    use crate::api_doc::extractors::Json;
    use crate::controller::{PageParam, PageRes};
    use axum::extract::State;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

    pub async fn create_entity(
        State(pool): State<ConnPool>,
        Json(new_entity): Json<GroupsPermission>,
    ) -> Result<Json<GroupsPermission>, AppError> {
        let mut connection = pool.get()?;
        let result = diesel::insert_into(groups_permissions)
            .values(new_entity)
            .returning(GroupsPermission::as_returning())
            .get_result(&mut connection)?;
        Ok(Json(result))
    }

    pub async fn get_entity_by_id(
        State(pool): State<ConnPool>,
        Path(id_param): Path<(i64, i64)>,
    ) -> Result<Json<GroupsPermission>, AppError> {
        let mut connection = pool.get()?;
        let result = groups_permissions
            .find(id_param)
            .select(GroupsPermission::as_select())
            .get_result(&mut connection)?;
        Ok(Json(result))
    }
    pub async fn delete_entity_by_id(
        State(pool): State<ConnPool>,
        Path(id_param): Path<(i64, i64)>,
    ) -> Result<Json<GroupsPermission>, AppError> {
        let mut connection = pool.get()?;
        let result = diesel::delete(groups_permissions.find(id_param))
            .returning(GroupsPermission::as_returning())
            .get_result(&mut connection)?;

        Ok(Json(result))
    }
    pub async fn get_entity_page(
        State(pool): State<ConnPool>,
        Json(page): Json<PageParam<GroupsPermissionBuilder>>,
    ) -> Result<Json<PageRes<GroupsPermission, GroupsPermissionBuilder>>, AppError> {
        let mut connection = pool.get()?;
        let off_lim = page.get_offset_limit();
        let mut statement = crate::schema::groups_permissions::dsl::groups_permissions.into_boxed();
        let mut count_statement =
            crate::schema::groups_permissions::dsl::groups_permissions.into_boxed();
        let filter = page.filters.clone();
        if let Some(filter_param) = filter.group_id {
            match filter_param.compare {
                Compare::NotEqual => {
                    statement = statement.filter(
                        crate::schema::groups_permissions::group_id
                            .ne(filter_param.compare_value.clone()),
                    );
                    count_statement = count_statement.filter(
                        crate::schema::groups_permissions::group_id.ne(filter_param.compare_value),
                    );
                }
                Compare::Equal => {
                    statement = statement.filter(
                        crate::schema::groups_permissions::group_id
                            .eq(filter_param.compare_value.clone()),
                    );
                    count_statement = count_statement.filter(
                        crate::schema::groups_permissions::group_id.eq(filter_param.compare_value),
                    );
                }
                Compare::Greater => {
                    statement = statement.filter(
                        crate::schema::groups_permissions::group_id
                            .gt(filter_param.compare_value.clone()),
                    );
                    count_statement = count_statement.filter(
                        crate::schema::groups_permissions::group_id.gt(filter_param.compare_value),
                    );
                }
                Compare::GreaterAndEqual => {
                    statement = statement.filter(
                        crate::schema::groups_permissions::group_id
                            .ge(filter_param.compare_value.clone()),
                    );
                    count_statement = count_statement.filter(
                        crate::schema::groups_permissions::group_id.ge(filter_param.compare_value),
                    );
                }
                Compare::Less => {
                    statement = statement.filter(
                        crate::schema::groups_permissions::group_id
                            .lt(filter_param.compare_value.clone()),
                    );
                    count_statement = count_statement.filter(
                        crate::schema::groups_permissions::group_id.lt(filter_param.compare_value),
                    );
                }
                Compare::LessAndEqual => {
                    statement = statement.filter(
                        crate::schema::groups_permissions::group_id
                            .le(filter_param.compare_value.clone()),
                    );
                    count_statement = count_statement.filter(
                        crate::schema::groups_permissions::group_id.le(filter_param.compare_value),
                    );
                }
            }
        }
        if let Some(filter_param) = filter.permission_id {
            match filter_param.compare {
                Compare::NotEqual => {
                    statement = statement.filter(
                        crate::schema::groups_permissions::permission_id
                            .ne(filter_param.compare_value.clone()),
                    );
                    count_statement = count_statement.filter(
                        crate::schema::groups_permissions::permission_id
                            .ne(filter_param.compare_value),
                    );
                }
                Compare::Equal => {
                    statement = statement.filter(
                        crate::schema::groups_permissions::permission_id
                            .eq(filter_param.compare_value.clone()),
                    );
                    count_statement = count_statement.filter(
                        crate::schema::groups_permissions::permission_id
                            .eq(filter_param.compare_value),
                    );
                }
                Compare::Greater => {
                    statement = statement.filter(
                        crate::schema::groups_permissions::permission_id
                            .gt(filter_param.compare_value.clone()),
                    );
                    count_statement = count_statement.filter(
                        crate::schema::groups_permissions::permission_id
                            .gt(filter_param.compare_value),
                    );
                }
                Compare::GreaterAndEqual => {
                    statement = statement.filter(
                        crate::schema::groups_permissions::permission_id
                            .ge(filter_param.compare_value.clone()),
                    );
                    count_statement = count_statement.filter(
                        crate::schema::groups_permissions::permission_id
                            .ge(filter_param.compare_value),
                    );
                }
                Compare::Less => {
                    statement = statement.filter(
                        crate::schema::groups_permissions::permission_id
                            .lt(filter_param.compare_value.clone()),
                    );
                    count_statement = count_statement.filter(
                        crate::schema::groups_permissions::permission_id
                            .lt(filter_param.compare_value),
                    );
                }
                Compare::LessAndEqual => {
                    statement = statement.filter(
                        crate::schema::groups_permissions::permission_id
                            .le(filter_param.compare_value.clone()),
                    );
                    count_statement = count_statement.filter(
                        crate::schema::groups_permissions::permission_id
                            .le(filter_param.compare_value),
                    );
                }
            }
        }
        let total_count = count_statement.count().get_result::<i64>(&mut connection)?;
        let res;
        let x_table = diesel_dynamic_schema::table(stringify!(groups_permissions));
        let order_column = x_table.column::<diesel::sql_types::Text, _>(page.order_column.clone());
        if page.is_desc {
            res = statement
                .offset(off_lim.0)
                .limit(off_lim.1)
                .order(order_column.desc())
                .select(GroupsPermission::as_select())
                .load(&mut connection)?;
        } else {
            res = statement
                .offset(off_lim.0)
                .limit(off_lim.1)
                .order(order_column.asc())
                .select(GroupsPermission::as_select())
                .load(&mut connection)?;
        }
        let page_res = PageRes::from_param_records_count(page, res, total_count);
        Ok(Json(page_res))
    }
}
