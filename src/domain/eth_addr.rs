use aide::OperationIo;
use alloy::primitives::Address;
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Schema, SchemaObject};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(OperationIo, Default, Debug, Clone, AsExpression, FromSqlRow, Hash, Eq, PartialEq)]
#[diesel(sql_type = Text)]
pub struct EthAddr(pub Address);

impl From<Address> for EthAddr {
    fn from(value: Address) -> Self {
        EthAddr(value)
    }
}

impl From<&Address> for EthAddr {
    fn from(value: &Address) -> Self {
        EthAddr(value.clone())
    }
}

impl std::fmt::Display for EthAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for EthAddr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for EthAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        String::deserialize(deserializer)
            .and_then(|string| {
                Address::from_str(&string).map_err(|err| {
                    Error::custom(format!("deserialize value:`{string}` failed,err:{err}"))
                })
            })
            .map(EthAddr)
    }
}

impl AsRef<Address> for EthAddr {
    fn as_ref(&self) -> &Address {
        &self.0
    }
}

impl Deref for EthAddr {
    type Target = Address;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EthAddr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl JsonSchema for EthAddr {
    fn schema_name() -> String {
        "EthAddr".to_owned()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}
use crate::db_models::DbType;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;

impl ToSql<Text, DbType> for EthAddr {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DbType>) -> serialize::Result {
        <String as ToSql<VarChar, DbType>>::to_sql(&self.0.to_string(), &mut out.reborrow())
    }
}

impl FromSql<Text, DbType> for EthAddr {
    fn from_sql(
        bytes: <DbType as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let string = <String as FromSql<VarChar, DbType>>::from_sql(bytes)?;
        let addr = Address::from_str(&string).map_err(Box::new)?;

        Ok(EthAddr(addr))
    }
}
