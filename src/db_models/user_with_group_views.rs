use chrono::{DateTime, Utc};
use derive_builder::ViewApiGen;
use diesel::{AsChangeset, Queryable, QueryableByName, Selectable};
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
    AsChangeset,
    Debug,
    ViewApiGen,
)]
#[diesel(table_name = crate::schema_view::user_with_group_views)]
#[diesel(check_for_backend(super::DbType))]
pub struct UserWithGroupView {
    pub id: i64,
    /// # Username
    /// in eth mode it will be address
    pub username: String,
    /// # password
    /// in eth mode it will be signature
    pub password: String,
    pub group_id: i64,
    pub tenantry: String,
    pub remark: Option<String>,
    pub update_time: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub update_by: Option<i64>,
    pub is_delete: bool,
    pub group_name: Option<String>,
}
