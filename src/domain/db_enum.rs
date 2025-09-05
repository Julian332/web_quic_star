use derive_more::{Display, Error};
use diesel::sql_types::VarChar;
use diesel::{AsExpression, FromSqlRow};
use diesel_enum::DbEnum;
use schemars::JsonSchema;

#[derive(Display, Error, Debug)]
pub struct EnumConvertError {
    msg: String,
}

impl EnumConvertError {
    fn not_found(msg: String) -> Self {
        Self { msg }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, DbEnum, JsonSchema)]
#[diesel(sql_type = VarChar)]
#[diesel_enum(error_fn = EnumConvertError::not_found)]
#[diesel_enum(error_type = EnumConvertError)]
pub enum Status {
    /// Will be represented as `"ready"`.
    Ready,
    /// Will be represented as `"pending"`.
    Pending,
}
