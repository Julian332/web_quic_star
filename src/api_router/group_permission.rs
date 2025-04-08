use crate::db_models::group_permission::GroupsPermission;
use crate::framework::api::{BoolOp, Compare};
use crate::framework::api_doc::{default_resp_docs, empty_resp_docs};
use crate::framework::auth::AuthBackend;
use crate::schema::groups_permissions::dsl::groups_permissions;
use aide::axum::routing::{delete_with, get_with, post_with};
use aide::axum::ApiRouter;
use axum::extract::Path;
use axum_login::permission_required;
pub fn group_permission_routes() -> ApiRouter {
    let router_add = ApiRouter::new().api_route(
        "/create_entity",
        post_with(web::create_entity, empty_resp_docs),
    );
    let router_read = ApiRouter::new()
        .api_route(
            "/get_entity_by_id/{group_id}/{permission_id}",
            get_with(web::get_entity_by_id, default_resp_docs::<GroupsPermission>),
        )
        .api_route(
            "/get_page",
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
        .merge(router_read)
        .merge(router_delete.route_layer(permission_required!(AuthBackend, "users_delete")))
}
pub(crate) mod web {
    use super::*;
    use crate::framework::api::{CompareValue, PageRes};
    use crate::framework::api::{DynFilter, PageParam};
    use crate::framework::db::Paginate;
    use crate::framework::errors::AppError;
    use crate::DB;
    use axum::Json;
    use diesel::prelude::*;

    pub async fn create_entity(
        Json(new_entity): Json<GroupsPermission>,
    ) -> Result<Json<GroupsPermission>, AppError> {
        let mut connection = DB.get()?;
        let result = diesel::insert_into(groups_permissions)
            .values(new_entity)
            .returning(GroupsPermission::as_returning())
            .get_result(&mut connection)?;
        Ok(Json(result))
    }

    pub async fn get_entity_by_id(
        Path(id_param): Path<(i64, i64)>,
    ) -> Result<Json<GroupsPermission>, AppError> {
        let result = groups_permissions
            .find(id_param)
            .select(GroupsPermission::as_select())
            .get_result(&mut DB.get()?)?;
        Ok(Json(result))
    }
    pub async fn delete_entity_by_id(
        Path(id_param): Path<(i64, i64)>,
    ) -> Result<Json<GroupsPermission>, AppError> {
        let result = diesel::delete(groups_permissions.find(id_param))
            .returning(GroupsPermission::as_returning())
            .get_result(&mut DB.get()?)?;

        Ok(Json(result))
    }
    pub async fn get_entity_page(
        Json(page): Json<PageParam<Vec<DynFilter>>>,
    ) -> Result<Json<PageRes<GroupsPermission, Vec<DynFilter>>>, AppError> {
        let mut connection = DB.get()?;
        let mut statement = groups_permissions.into_boxed();
        let x_table = diesel_dynamic_schema::table(stringify!(groups_permissions));

        let filter = page.filters.clone();
        for f in filter {
            match f.compare_value {
                CompareValue::Bool(compare_value) => {
                    let filter_column = x_table.column::<diesel::sql_types::Bool, _>(f.column);
                    match f.op.unwrap_or_default() {
                        BoolOp::And => match f.compare.unwrap_or_default() {
                            Compare::NotEqual => {
                                statement = statement.filter(filter_column.ne(compare_value));
                            }
                            Compare::Equal => {
                                statement = statement.filter(filter_column.eq(compare_value));
                            }
                            Compare::Greater => {
                                statement = statement.filter(filter_column.gt(compare_value));
                            }
                            Compare::GreaterAndEqual => {
                                statement = statement.filter(filter_column.ge(compare_value));
                            }
                            Compare::Less => {
                                statement = statement.filter(filter_column.lt(compare_value));
                            }
                            Compare::LessAndEqual => {
                                statement = statement.filter(filter_column.le(compare_value));
                            }
                        },
                        BoolOp::Or => match f.compare.unwrap_or_default() {
                            Compare::NotEqual => {
                                statement = statement.or_filter(filter_column.ne(compare_value));
                            }
                            Compare::Equal => {
                                statement = statement.or_filter(filter_column.eq(compare_value));
                            }
                            Compare::Greater => {
                                statement = statement.or_filter(filter_column.gt(compare_value));
                            }
                            Compare::GreaterAndEqual => {
                                statement = statement.or_filter(filter_column.ge(compare_value));
                            }
                            Compare::Less => {
                                statement = statement.or_filter(filter_column.lt(compare_value));
                            }
                            Compare::LessAndEqual => {
                                statement = statement.or_filter(filter_column.le(compare_value));
                            }
                        },
                    }
                }
                CompareValue::Float(compare_value) => {
                    let filter_column = x_table.column::<diesel::sql_types::Float8, _>(f.column);
                    match f.op.unwrap_or_default() {
                        BoolOp::And => match f.compare.unwrap_or_default() {
                            Compare::NotEqual => {
                                statement = statement.filter(filter_column.ne(compare_value));
                            }
                            Compare::Equal => {
                                statement = statement.filter(filter_column.eq(compare_value));
                            }
                            Compare::Greater => {
                                statement = statement.filter(filter_column.gt(compare_value));
                            }
                            Compare::GreaterAndEqual => {
                                statement = statement.filter(filter_column.ge(compare_value));
                            }
                            Compare::Less => {
                                statement = statement.filter(filter_column.lt(compare_value));
                            }
                            Compare::LessAndEqual => {
                                statement = statement.filter(filter_column.le(compare_value));
                            }
                        },
                        BoolOp::Or => match f.compare.unwrap_or_default() {
                            Compare::NotEqual => {
                                statement = statement.or_filter(filter_column.ne(compare_value));
                            }
                            Compare::Equal => {
                                statement = statement.or_filter(filter_column.eq(compare_value));
                            }
                            Compare::Greater => {
                                statement = statement.or_filter(filter_column.gt(compare_value));
                            }
                            Compare::GreaterAndEqual => {
                                statement = statement.or_filter(filter_column.ge(compare_value));
                            }
                            Compare::Less => {
                                statement = statement.or_filter(filter_column.lt(compare_value));
                            }
                            Compare::LessAndEqual => {
                                statement = statement.or_filter(filter_column.le(compare_value));
                            }
                        },
                    }
                }
                CompareValue::String(compare_value) => {
                    let filter_column = x_table.column::<diesel::sql_types::Text, _>(f.column);
                    match f.op.unwrap_or_default() {
                        BoolOp::And => match f.compare.unwrap_or_default() {
                            Compare::NotEqual => {
                                statement = statement.filter(filter_column.ne(compare_value));
                            }
                            Compare::Equal => {
                                statement = statement.filter(filter_column.eq(compare_value));
                            }
                            Compare::Greater => {
                                statement = statement.filter(filter_column.gt(compare_value));
                            }
                            Compare::GreaterAndEqual => {
                                statement = statement.filter(filter_column.ge(compare_value));
                            }
                            Compare::Less => {
                                statement = statement.filter(filter_column.lt(compare_value));
                            }
                            Compare::LessAndEqual => {
                                statement = statement.filter(filter_column.le(compare_value));
                            }
                        },
                        BoolOp::Or => match f.compare.unwrap_or_default() {
                            Compare::NotEqual => {
                                statement = statement.or_filter(filter_column.ne(compare_value));
                            }
                            Compare::Equal => {
                                statement = statement.or_filter(filter_column.eq(compare_value));
                            }
                            Compare::Greater => {
                                statement = statement.or_filter(filter_column.gt(compare_value));
                            }
                            Compare::GreaterAndEqual => {
                                statement = statement.or_filter(filter_column.ge(compare_value));
                            }
                            Compare::Less => {
                                statement = statement.or_filter(filter_column.lt(compare_value));
                            }
                            Compare::LessAndEqual => {
                                statement = statement.or_filter(filter_column.le(compare_value));
                            }
                        },
                    }
                }
                CompareValue::BigDecimal(compare_value) => {
                    let filter_column = x_table.column::<diesel::sql_types::Decimal, _>(f.column);
                    match f.op.unwrap_or_default() {
                        BoolOp::And => match f.compare.unwrap_or_default() {
                            Compare::NotEqual => {
                                statement = statement.filter(filter_column.ne(compare_value));
                            }
                            Compare::Equal => {
                                statement = statement.filter(filter_column.eq(compare_value));
                            }
                            Compare::Greater => {
                                statement = statement.filter(filter_column.gt(compare_value));
                            }
                            Compare::GreaterAndEqual => {
                                statement = statement.filter(filter_column.ge(compare_value));
                            }
                            Compare::Less => {
                                statement = statement.filter(filter_column.lt(compare_value));
                            }
                            Compare::LessAndEqual => {
                                statement = statement.filter(filter_column.le(compare_value));
                            }
                        },
                        BoolOp::Or => match f.compare.unwrap_or_default() {
                            Compare::NotEqual => {
                                statement = statement.or_filter(filter_column.ne(compare_value));
                            }
                            Compare::Equal => {
                                statement = statement.or_filter(filter_column.eq(compare_value));
                            }
                            Compare::Greater => {
                                statement = statement.or_filter(filter_column.gt(compare_value));
                            }
                            Compare::GreaterAndEqual => {
                                statement = statement.or_filter(filter_column.ge(compare_value));
                            }
                            Compare::Less => {
                                statement = statement.or_filter(filter_column.lt(compare_value));
                            }
                            Compare::LessAndEqual => {
                                statement = statement.or_filter(filter_column.le(compare_value));
                            }
                        },
                    }
                }
            };
        }
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
