use aide::OperationIo;
use alloy::primitives::{Address, address};
use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, From, FromStr, Into};
use schemars::generate::SchemaGenerator;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::convert::Into;
#[derive(
    OperationIo,
    Default,
    Debug,
    Clone,
    AsExpression,
    FromSqlRow,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    From,
    AsMut,
    AsRef,
    Display,
    Deref,
    DerefMut,
    FromStr,
    Into,
    derive_more::BitAnd,
    derive_more::BitAndAssign,
    derive_more::BitOr,
    derive_more::BitOrAssign,
    derive_more::BitXor,
    derive_more::BitXorAssign,
    derive_more::Not,
    derive_more::Index,
    derive_more::IndexMut,
    derive_more::IntoIterator,
    derive_more::LowerHex,
    derive_more::UpperHex,
)]
#[diesel(sql_type = Text)]
pub struct EthAddr(#[into_iterator(owned, ref, ref_mut)] pub Address);

use std::str::FromStr;
#[allow(unused)]
#[derive(Debug)]
pub struct EthAddrs {
    pub uni_router2_addr: EthAddr,
    pub weth_addr: EthAddr,
    pub usdt_addr: EthAddr,
}

impl Default for EthAddrs {
    fn default() -> Self {
        #[cfg(not(feature = "dev"))]
        {
            EthAddrs {
                uni_router2_addr: address!("0x1689E7B1F10000AE47eBfE339a4f69dECd19F602").into(),
                weth_addr: address!("0x4200000000000000000000000000000000000006").into(),
                usdt_addr: address!("0x2ab0c976EB9551c5d18e80178C92bAf17391Bc79").into(),
            }
        }
        #[cfg(feature = "dev")]
        {
            EthAddrs {
                uni_router2_addr: address!("0x1689E7B1F10000AE47eBfE339a4f69dECd19F602").into(),
                weth_addr: address!("0x4200000000000000000000000000000000000006").into(),
                usdt_addr: address!("0x2ab0c976EB9551c5d18e80178C92bAf17391Bc79").into(),
            }
        }
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

impl JsonSchema for EthAddr {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("EthAddr")
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
use schemars::{JsonSchema, Schema, json_schema};

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

impl From<&Address> for EthAddr {
    fn from(value: &Address) -> Self {
        EthAddr(*value)
    }
}
impl From<&EthAddr> for  Address {
    fn from(value: &EthAddr) -> Self {
        value.0
    }
}

