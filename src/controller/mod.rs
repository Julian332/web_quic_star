pub mod group;
pub mod group_permission;
pub mod permission;
pub mod user;

use aide::OperationIo;
use diesel::QueryableByName;
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Schema, SchemaObject};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

const LOGIN_URL: &str = "/auth/login";
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct PageParam<T: Default> {
    pub filters: T,
    pub page_no: i64,
    pub page_size: i64,
    pub order_column: String,
    pub is_desc: bool,
}

impl<T: Default> Default for PageParam<T> {
    fn default() -> Self {
        PageParam {
            filters: T::default(),
            page_no: 1,
            page_size: 10,
            order_column: "create_time".to_string(),
            is_desc: true,
        }
    }
}

impl<T: Default> PageParam<T> {
    pub fn get_offset_limit(&self) -> (i64, i64) {
        ((self.page_no - 1) * self.page_size, self.page_size)
    }
}
#[derive(QueryableByName)]
pub struct Count {
    #[sql_type = "diesel::sql_types::BigInt"]
    pub count: i64,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Default)]
pub enum Compare {
    NotEqual,
    #[default]
    Equal,
    Greater,
    GreaterAndEqual,
    Less,
    LessAndEqual,
}

impl Compare {
    pub fn to_ident(self) -> String {
        match self {
            Compare::NotEqual => "ne",
            Compare::Equal => "eq",
            Compare::Greater => "gt",
            Compare::GreaterAndEqual => "ge",
            Compare::Less => "lt",
            Compare::LessAndEqual => "le",
        }
        .to_string()
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Default)]
pub struct Filter<T> {
    pub compare: Compare,
    pub compare_value: T,
}

#[derive(Debug, Serialize, Deserialize, Default, JsonSchema)]
#[serde(default)]
pub struct PageRes<T: Default, TBuilder: Default> {
    pub page_no: i64,
    pub page_size: i64,
    pub records: Vec<T>,
    pub total_page: i64,
    pub filters: TBuilder,
}

impl<T: Default, TBuilder: Default> PageRes<T, TBuilder> {
    pub fn from_param_records(param: PageParam<TBuilder>, records: Vec<T>) -> PageRes<T, TBuilder> {
        PageRes {
            page_no: param.page_no,
            page_size: param.page_size,
            records,
            total_page: -1,
            filters: param.filters,
        }
    }
    pub fn from_param_records_count(
        param: PageParam<TBuilder>,
        records: Vec<T>,
        total_count: i64,
    ) -> PageRes<T, TBuilder> {
        if total_count % param.page_size == 0 {
            PageRes {
                page_no: param.page_no,
                page_size: param.page_size,
                records,
                total_page: total_count / param.page_size,
                filters: param.filters,
            }
        } else {
            PageRes {
                page_no: param.page_no,
                page_size: param.page_size,
                records,
                total_page: total_count / param.page_size + 1,
                filters: param.filters,
            }
        }
    }
}

// #[macro_export]
// macro_rules! web_fn_gen {
//     ($table:ident ,$new:ident, $result:ident) => {
//         async fn create_entity(
//             State(pool): State<Pool<ConnectionManager<PgConnection>>>,
//             Json(new_entity): Json<$new>,
//         ) -> Result<Json<$result>, String> {
//             let mut connection = pool.get().unwrap();
//
//             let result = diesel::insert_into($table)
//                 .values(new_entity)
//                 .returning($result::as_returning())
//                 .get_result(&mut connection)
//                 .expect("Error saving new entity");
//
//             Ok(Json::from(result))
//         }
//
//         async fn update_entity_by_id(
//             State(pool): State<Pool<ConnectionManager<PgConnection>>>,
//             Path(id_param): Path<i64>,
//             Json(new): Json<$new>,
//         ) -> Result<Json<$result>, String> {
//             let mut connection = pool.get().unwrap();
//             let result = diesel::update($table.find(id_param))
//                 .set(&new)
//                 .returning($result::as_returning())
//                 .get_result(&mut connection)
//                 .expect("Error update  entity");
//             Ok(Json(result))
//         }
//
//         async fn get_entity_by_id(
//             State(pool): State<Pool<ConnectionManager<PgConnection>>>,
//             Path(id_param): Path<i64>,
//         ) -> Result<Json<$result>, String> {
//             let mut connection = pool.get().unwrap();
//             let result = $table
//                 .find(id_param)
//                 .select($result::as_select())
//                 .get_result(&mut connection)
//                 .expect("get entity by id failed");
//             Ok(Json(result))
//         }
//
//         async fn delete_entity_by_id(
//             State(pool): State<Pool<ConnectionManager<PgConnection>>>,
//             Path(id_param): Path<i64>,
//         ) -> Result<Json<$result>, String> {
//             let mut connection = pool.get().unwrap();
//             let result = diesel::update($table.find(id_param))
//                 .set(crate::schema::$table::is_delete.eq(true))
//                 .returning($result::as_returning())
//                 .get_result(&mut connection)
//                 .expect("Error delete  entity");
//             Ok(Json(result))
//         }
//
//         // async fn get_entity_page(
//         //     State(pool): State<Pool<ConnectionManager<PgConnection>>>,
//         //     Json(page): Json<PageParam<$filter>>,
//         // ) -> Result<Json<PageRes<$result, $filter>>, String> {
//         //     let mut connection = pool.get().unwrap();
//         //     let off_lim = page.get_offset_limit();
//         //     let res;
//         //     let x_table = table(stringify!($table));
//         //     let order_column = x_table.column::<Text, _>(page.order_column.clone());
//         //     if page.is_desc {
//         //         res = $table
//         //             .offset(off_lim.0)
//         //             .limit(off_lim.1)
//         //             .order(order_column.desc())
//         //             .select($result::as_select())
//         //             .load(&mut connection)
//         //             .expect("Error loading page");
//         //     } else {
//         //         res = $table
//         //             .offset(off_lim.0)
//         //             .limit(off_lim.1)
//         //             .order(order_column.asc())
//         //             .select($result::as_select())
//         //             .load(&mut connection)
//         //             .expect("Error loading page");
//         //     }
//         //
//         //     let page_res = PageRes::from_param_records(page, res);
//         //     Ok(Json(page_res))
//         // }
//     };
// }

// #[macro_export]
// macro_rules! web_router_gen {
//     ($table:ident ,$new:ident, $result:ident) => {
//         use crate::api_auth::login_impl::AuthBackend;
//         use crate::controller::LOGIN_URL;
//         use crate::openapi::{default_resp_docs, empty_resp_docs};
//         use crate::schema::$table::dsl::$table;
//         use crate::web_fn_gen;
//         use aide::axum::routing::{delete_with, get_with, post_with, put_with};
//         use aide::axum::ApiRouter;
//         use axum::extract::{Path, State};
//         use axum::response::Json;
//         use axum_login::login_required;
//         use diesel::r2d2::{ConnectionManager, Pool};
//         use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
//
//         pub(crate) fn web_routes(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
//             ApiRouter::new()
//                 .api_route(
//                     "/create_entity",
//                     post_with(create_entity, empty_resp_docs),
//                     // .get_with(list_todos, empty_resp_docs),
//                 )
//                 .api_route(
//                     "/get_entity_by_id/:id",
//                     get_with(get_entity_by_id, default_resp_docs::<$result>),
//                     // .delete_with(delete_todo, empty_resp_docs),
//                 )
//                 .api_route(
//                     "/update_entity_by_id/:id",
//                     put_with(update_entity_by_id, default_resp_docs::<$result>),
//                 )
//                 .api_route(
//                     "/delete_entity_by_id/:id",
//                     delete_with(delete_entity_by_id, default_resp_docs::<$result>),
//                 )
//                 .api_route(
//                     "/get_entity_page",
//                     post_with(web::get_entity_page, empty_resp_docs),
//                 )
//                 .with_state(conn_pool)
//                 .route_layer(login_required!(AuthBackend, login_url = LOGIN_URL))
//         }
//
//         web_fn_gen!($table, $new, $result);
//     };
// }
