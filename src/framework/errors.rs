use crate::CURRENT_REQ;
use aide::OperationIo;
use anyhow::anyhow;
use axum::response::IntoResponse;
use derive_more::{Display, Error};
use schemars::JsonSchema;
use serde::Serialize;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;
pub trait OkOrErr<T> {
    fn ok_or_err(self) -> Result<T, NoneError>;
}

impl<T> OkOrErr<T> for Option<T> {
    fn ok_or_err(self) -> Result<T, NoneError> {
        self.ok_or(NoneError)
    }
}
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
        let uuid = CURRENT_REQ
            .try_with(|x| x.0.clone())
            .unwrap_or_else(|_| Uuid::now_v7());
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
        tracing::error!("err: {self:?};");
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

    let _result: Result<i32, NoneError> = Some(1).ok_or_err();
}

impl AppError {
    pub fn new(error: &str) -> Self {
        let uuid = CURRENT_REQ
            .try_with(|x| x.0.clone())
            .unwrap_or_else(|_| Uuid::now_v7());
        Self {
            error: anyhow::anyhow!("{error}"),
            error_id: uuid,
        }
    }
}
