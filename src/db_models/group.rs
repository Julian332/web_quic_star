use crate::framework::auth::AuthPermission;
use chrono::{DateTime, Utc};
use derive_builder::WebApiGen;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Debug, Selectable, Serialize, Deserialize, JsonSchema, AsChangeset, Insertable,
)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(super::DbType))]
#[serde(default)]
pub struct NewGroup {
    pub name: String,
    pub remark: Option<String>,
    pub update_time: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub update_by: Option<i64>,
    pub is_delete: bool,
    pub permissions: Vec<AuthPermission>,
}

impl Default for NewGroup {
    fn default() -> Self {
        NewGroup {
            name: "name".to_string(),
            remark: None,
            update_time: None,
            create_time: Default::default(),
            create_by: 0,
            update_by: None,
            is_delete: false,
            permissions: vec![],
        }
    }
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
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(super::DbType))]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub remark: Option<String>,
    pub update_time: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub update_by: Option<i64>,
    pub is_delete: bool,
    pub permissions: Vec<AuthPermission>,
}
