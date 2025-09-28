use crate::CURRENT_REQ_HEADER;
use aide::OperationIo;
use anyhow::anyhow;
use axum::response::IntoResponse;
use derive_more::{Display, Error};
use schemars::{JsonSchema, json_schema};
use serde::Serialize;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use tracing::info;
use uuid::Uuid;

#[allow(unused)]
#[derive(Debug, Display, Error)]
pub struct NoneError;
/// A default error response for most API errors.
#[derive(Debug, OperationIo, Serialize, JsonSchema)]
pub struct AppError {
    /// A unique error ID.
    error_id: Uuid,
    #[serde(skip)]
    error: anyhow::Error,
}
impl<T: Error + Send + Sync + 'static> From<T> for AppError {
    fn from(value: T) -> Self {
        let value = anyhow!(value);
        let uuid = Uuid::now_v7();
        tracing::debug!("Error:{value:?}; Error ID:{uuid};");
        let app_error = Self {
            error: value.into(),
            error_id: uuid,
        };
        app_error
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match CURRENT_REQ_HEADER.try_with(|x| {
            tracing::debug!("request header: {x:?};");
        }) {
            Ok(_) => {
                info!("web req err: {self:?};");
            }
            Err(_) => {
                tracing::debug!("internal err: {self:?};");
            }
        }
        axum::Json(self).into_response()
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error:{}, +error_id:{}", self.error, self.error_id)
    }
}
#[test]
fn test_display_error() {
    let x: AppError = BlockError::InvalidSignature.into();
    println!("{:?}", x);
    // let error = AppError::new("eee");
    // println!("{}", error);
    // println!("{:?}", error);
    // println!("{:?}", serde_json::to_string(&error));
    use alloy::rpc::types::BlockError;
}

impl AppError {
    pub fn new(error: &str) -> Self {
        Self {
            error: anyhow::anyhow!("{error}"),
            error_id: Uuid::new_v4(),
        }
    }
}
