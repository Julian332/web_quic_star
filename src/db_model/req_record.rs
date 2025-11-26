use crate::framework::db::DbType;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use web_api_gen::WebApiGen;

#[derive(
    Queryable,
    Debug,
    Selectable,
    Serialize,
    Deserialize,
    JsonSchema,
    AsChangeset,
    Insertable,
    Default,
)]
#[diesel(table_name = crate::schema::req_records)]
#[diesel(check_for_backend(DbType))]
#[serde(default)]
pub struct NewReqRecord {
    pub username: Option<String>,
    pub req_id: String,
    pub req_body: Option<String>,
    pub path: String,
    pub status_code: String,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
}

#[derive(
    Queryable,
    Debug,
    Identifiable,
    Selectable,
    WebApiGen,
    Serialize,
    Deserialize,
    JsonSchema,
    Default,
)]
#[diesel(table_name = crate::schema::req_records)]
#[diesel(check_for_backend(DbType))]
pub struct ReqRecord {
    pub id: i64,
    pub username: Option<String>,
    pub req_id: String,
    pub req_body: Option<String>,
    pub path: String,
    pub status_code: String,
    pub update_time: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub update_by: Option<i64>,
    pub is_delete: bool,
}
