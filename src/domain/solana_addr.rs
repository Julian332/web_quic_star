use aide::OperationIo;
use anchor_client::anchor_lang::prelude::Pubkey;
use schemars::generate::SchemaGenerator;
use schemars::{JsonSchema, Schema, json_schema};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[allow(unused)]
#[derive(Debug)]
pub struct SolAddrs {}

impl Default for SolAddrs {
    fn default() -> Self {
        #[cfg(feature = "dev")]
        {
            SolAddrs {}
        }
        #[cfg(not(feature = "dev"))]
        {
            SolAddrs {}
        }
    }
}
#[derive(
    OperationIo,
    Default,
    Debug,
    Clone,
    AsExpression,
    FromSqlRow,
    Copy,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
)]
#[diesel(sql_type = Text)]
pub struct SolAddr(pub Pubkey);

impl Display for SolAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for SolAddr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for SolAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use std::str::FromStr;
        String::deserialize(deserializer)
            .and_then(|string| {
                Pubkey::from_str(&string).map_err(|err| {
                    Error::custom(format!("deserialize value:`{string}` failed,err:{err}"))
                })
            })
            .map(SolAddr)
    }
}

impl JsonSchema for SolAddr {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("SolAddr")
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
        })
    }
}

use crate::db_models::DbType;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;

impl ToSql<Text, DbType> for SolAddr {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DbType>) -> serialize::Result {
        <String as ToSql<VarChar, DbType>>::to_sql(&self.0.to_string(), &mut out.reborrow())
    }
}

impl FromSql<Text, DbType> for SolAddr {
    fn from_sql(
        bytes: <DbType as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let string = <String as FromSql<VarChar, DbType>>::from_sql(bytes)?;
        let pubkey = Pubkey::from_str(&string).map_err(Box::new)?;

        Ok(SolAddr(pubkey))
    }
}
impl From<Pubkey> for SolAddr {
    fn from(value: Pubkey) -> Self {
        SolAddr(value)
    }
}

impl From<&Pubkey> for SolAddr {
    fn from(value: &Pubkey) -> Self {
        SolAddr(*value)
    }
}
impl AsRef<Pubkey> for SolAddr {
    fn as_ref(&self) -> &Pubkey {
        &self.0
    }
}

impl AsMut<Pubkey> for SolAddr {
    fn as_mut(&mut self) -> &mut Pubkey {
        &mut self.0
    }
}

impl Deref for SolAddr {
    type Target = Pubkey;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SolAddr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
