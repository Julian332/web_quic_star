// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use derive_builder::WebApiGen;
use diesel::{AsChangeset, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(clippy::all)]
/// example:{group_id: 1,permission_id: 1}
#[derive(Deserialize, Serialize, JsonSchema, Default, Clone)]
pub struct GroupsPermissionBuilder {
    pub group_id: ::derive_builder::export::core::option::Option<Filter<i64>>,
    pub permission_id: ::derive_builder::export::core::option::Option<Filter<i64>>,
}

use crate::db_models::group_permission::GroupsPermission;
use crate::db_models::ConnPool;
use crate::framework::api::Filter;
use crate::framework::api::LOGIN_URL;
use crate::framework::api::{BoolOp, Compare};
use crate::framework::api_doc::{default_resp_docs, empty_resp_docs};
use crate::framework::auth::AuthBackend;
use crate::schema::groups_permissions::dsl::groups_permissions;
use crate::AppRes;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use aide::axum::ApiRouter;
use axum::extract::Path;
use axum_login::permission_required;
use diesel::r2d2::{ConnectionManager, Pool};

pub(crate) mod web {
    use super::*;
    use crate::framework::api::PageRes;
    use crate::framework::api::{DynFilter, PageParam};
    use crate::framework::api_doc::errors::AppError;
    use crate::framework::db::Paginate;
    use axum::extract::State;
    use axum::Json;
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
        let result = groups_permissions
            .find(id_param)
            .select(GroupsPermission::as_select())
            .get_result(&mut pool.get()?)?;
        Ok(Json(result))
    }
    pub async fn delete_entity_by_id(
        State(pool): State<ConnPool>,
        Path(id_param): Path<(i64, i64)>,
    ) -> Result<Json<GroupsPermission>, AppError> {
        let result = diesel::delete(groups_permissions.find(id_param))
            .returning(GroupsPermission::as_returning())
            .get_result(&mut pool.get()?)?;

        Ok(Json(result))
    }
    pub async fn get_entity_page(
        State(pool): State<ConnPool>,
        Json(page): Json<PageParam<Vec<DynFilter>>>,
    ) -> Result<Json<PageRes<GroupsPermission, Vec<DynFilter>>>, AppError> {
        let mut connection = pool.get()?;
        let mut statement = crate::schema::groups_permissions::dsl::groups_permissions.into_boxed();
        let x_table = diesel_dynamic_schema::table(stringify!(groups_permissions));

        let filter = page.filters.clone();
        for f in filter {
            let filter_column = x_table.column::<diesel::sql_types::Text, _>(f.column);
            match f.op.unwrap_or_default() {
                BoolOp::And => match f.compare.unwrap_or_default() {
                    Compare::NotEqual => {
                        statement = statement.filter(filter_column.ne(f.value));
                    }
                    Compare::Equal => {
                        statement = statement.filter(filter_column.eq(f.value));
                    }
                    Compare::Greater => {
                        statement = statement.filter(filter_column.gt(f.value));
                    }
                    Compare::GreaterAndEqual => {
                        statement = statement.filter(filter_column.ge(f.value));
                    }
                    Compare::Less => {
                        statement = statement.filter(filter_column.lt(f.value));
                    }
                    Compare::LessAndEqual => {
                        statement = statement.filter(filter_column.le(f.value));
                    }
                },
                BoolOp::Or => match f.compare.unwrap_or_default() {
                    Compare::NotEqual => {
                        statement = statement.or_filter(filter_column.ne(f.value));
                    }
                    Compare::Equal => {
                        statement = statement.or_filter(filter_column.eq(f.value));
                    }
                    Compare::Greater => {
                        statement = statement.or_filter(filter_column.gt(f.value));
                    }
                    Compare::GreaterAndEqual => {
                        statement = statement.or_filter(filter_column.ge(f.value));
                    }
                    Compare::Less => {
                        statement = statement.or_filter(filter_column.lt(f.value));
                    }
                    Compare::LessAndEqual => {
                        statement = statement.or_filter(filter_column.le(f.value));
                    }
                },
            }
        }
        // if let Some(filter_param) = filter.group_id {
        //     match filter_param.compare {
        //         Compare::NotEqual => {
        //             statement = statement.filter(
        //                 crate::schema::groups_permissions::group_id
        //                     .ne(filter_param.compare_value.clone()),
        //             );
        //         }
        //         Compare::Equal => {
        //             statement = statement.filter(
        //                 crate::schema::groups_permissions::group_id
        //                     .eq(filter_param.compare_value.clone()),
        //             );
        //         }
        //         Compare::Greater => {
        //             statement = statement.filter(
        //                 crate::schema::groups_permissions::group_id
        //                     .gt(filter_param.compare_value.clone()),
        //             );
        //         }
        //         Compare::GreaterAndEqual => {
        //             statement = statement.filter(
        //                 crate::schema::groups_permissions::group_id
        //                     .ge(filter_param.compare_value.clone()),
        //             );
        //         }
        //         Compare::Less => {
        //             statement = statement.filter(
        //                 crate::schema::groups_permissions::group_id
        //                     .lt(filter_param.compare_value.clone()),
        //             );
        //         }
        //         Compare::LessAndEqual => {
        //             statement = statement.filter(
        //                 crate::schema::groups_permissions::group_id
        //                     .le(filter_param.compare_value.clone()),
        //             );
        //         }
        //     }
        // }
        // if let Some(filter_param) = filter.permission_id {
        //     match filter_param.compare {
        //         Compare::NotEqual => {
        //             statement = statement.filter(
        //                 crate::schema::groups_permissions::permission_id
        //                     .ne(filter_param.compare_value.clone()),
        //             );
        //         }
        //         Compare::Equal => {
        //             statement = statement.filter(
        //                 crate::schema::groups_permissions::permission_id
        //                     .eq(filter_param.compare_value.clone()),
        //             );
        //         }
        //         Compare::Greater => {
        //             statement = statement.filter(
        //                 crate::schema::groups_permissions::permission_id
        //                     .gt(filter_param.compare_value.clone()),
        //             );
        //         }
        //         Compare::GreaterAndEqual => {
        //             statement = statement.filter(
        //                 crate::schema::groups_permissions::permission_id
        //                     .ge(filter_param.compare_value.clone()),
        //             );
        //         }
        //         Compare::Less => {
        //             statement = statement.filter(
        //                 crate::schema::groups_permissions::permission_id
        //                     .lt(filter_param.compare_value.clone()),
        //             );
        //         }
        //         Compare::LessAndEqual => {
        //             statement = statement.filter(
        //                 crate::schema::groups_permissions::permission_id
        //                     .le(filter_param.compare_value.clone()),
        //             );
        //         }
        //     }
        // };
        let order_column = x_table.column::<diesel::sql_types::Text, _>(page.order_column.clone());
        let res = if page.is_desc {
            statement
                .order(order_column.desc())
                .select(GroupsPermission::as_select())
                .paginate(page.page_no, page.page_size)
                .load_and_count_pages(&mut connection)?
        } else {
            statement
                .order(order_column.asc())
                .select(GroupsPermission::as_select())
                .paginate(page.page_no, page.page_size)
                .load_and_count_pages(&mut connection)?
        };
        let page_res = PageRes::from_param_records_count(page, res.0, res.1);
        Ok(Json(page_res))
    }
}

pub fn group_permission_routes(conn_pool: ConnPool) -> ApiRouter {
    let router_add = ApiRouter::new().api_route(
        "/create_entity",
        post_with(
            crate::api::group_permission::web::create_entity,
            empty_resp_docs,
        ),
    );
    let router_read = ApiRouter::new()
        .api_route(
            "/get_entity_by_id/{group_id}/{permission_id}",
            get_with(web::get_entity_by_id, default_resp_docs::<GroupsPermission>),
        )
        .api_route(
            "/get_entity_page",
            post_with(web::get_entity_page, empty_resp_docs),
        );
    let router_delete = ApiRouter::new().api_route(
        "/delete_entity_by_id/{group_id}/{permission_id}",
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
