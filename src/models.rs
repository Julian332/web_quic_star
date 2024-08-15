// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use crate::domain::param_models::{OrderType, SellBuy};
use bigdecimal::BigDecimal;
use chrono::offset::Utc;
use chrono::DateTime;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::serialize::ToSql;
use diesel::Queryable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// #[derive(Queryable, Debug, Serialize, Deserialize, Default)]
// struct EthAddress(Address);


#[derive(Debug)]
#[derive(Queryable)]
#[diesel(table_name = crate::schema::following_order)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FollowingOrder {
  pub id: i64,
  pub deleted: bool,
  pub create_time: DateTime<Utc>,
  pub update_time: Option<DateTime<Utc>>,
}


#[derive(
  Queryable,
  Debug,
  Serialize,
  Deserialize,
  Default,
  JsonSchema,
  Selectable,
  Identifiable,
  AsChangeset,
  Clone
)]
#[diesel(table_name = crate::schema::addr_subscribes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AddrSubscribes {
  pub id: i64,
  pub deleted: bool,
  pub create_time: DateTime<Utc>,
  pub update_time: Option<DateTime<Utc>>,
  pub following_addr: String,
  pub subscribers: Vec<Option<String>>,
}
// #[derive(Queryable, Selectable, Insertable, Debug)]
// #[diesel(table_name = crate::schema::posts)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct Post {
//   pub id: i32,
//   pub title: String,
//   pub body: String,
//   pub published: bool,
// }

#[derive(
  Queryable,
  Debug,
  Serialize,
  Deserialize,
  Default,
  JsonSchema,
  Selectable,
  Identifiable,
  AsChangeset,
  Clone
)]
#[diesel(table_name = crate::schema::tg_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TgUser {
  pub id: i64,
  pub deleted: bool,
  // #[serde(with = "chrono_datetime_as_bson_datetime")]
  pub create_time: DateTime<Utc>,
  pub update_time: Option<DateTime<Utc>>,
  pub address: String,
  pub private_key: Option<String>,
  pub fee_staged: Option<BigDecimal>,
  pub fee_received: Option<BigDecimal>,
  pub parent: Option<String>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, Default, JsonSchema, Insertable, Selectable,AsChangeset)]
#[diesel(table_name = crate::schema::tg_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTgUser {
  // #[serde(with = "chrono_datetime_as_bson_datetime")]

  pub address: String,
  pub private_key: Option<String>,
  pub parent: Option<String>,
}

#[derive(
  Queryable,
  Debug,
  Serialize,
  Deserialize,
  JsonSchema,
  Insertable,
  Selectable,
  Identifiable,
  AsChangeset
)]
#[diesel(table_name = crate::schema::trading_order)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TradingOrder {
  pub id: i64,
  pub deleted: bool,
  pub create_time: DateTime<Utc>,
  pub update_time: Option<DateTime<Utc>>,
  pub sell_or_buy: SellBuy,
  pub target_token: String,
  pub from_token: String,
  pub trading_uer: i64,
  pub boost_mode: bool,
  pub mev_protected: bool,
  pub priority_fee: Option<BigDecimal>,
  pub is_succeed: Option<bool>,
  pub tx_hash: Option<String>,
  pub tx_receipt: Option<serde_json::Value>,
  pub target_amount: Option<BigDecimal>,
  pub from_token_amount: BigDecimal,
  pub pending_target_price: Option<BigDecimal>,
  pub expire_at: Option<DateTime<Utc>>,
  pub fee: Option<BigDecimal>,
  pub order_type: OrderType,
  pub slippage: Option<BigDecimal>,
  pub user_addr: String,
}




