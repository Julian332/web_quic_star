use aide::OperationIo;
use axum::{http::StatusCode, response::IntoResponse};
use derive_more::{Display, Error};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;
/// A default error response for most API errors.
#[derive(Debug, Serialize, JsonSchema, Deserialize, OperationIo)]
pub struct AppError {
    /// An error message.
    error: String,
    /// A unique error ID.
    error_id: Uuid,
    #[serde(skip)]
    status: StatusCode,
    /// Optional Additional error details.
    #[serde(skip_serializing_if = "Option::is_none")]
    error_details: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_origin_position: Option<String>,
}

#[allow(unused)]
#[derive(Debug, Display, Error)]
struct NoneError;
// impl Deref for AppError {
//     type Target = dyn StdError + Send + Sync + 'static;
//
//     fn deref(&self) -> &Self::Target {
//         unsafe { ErrorImpl::error(self.inner.by_ref()) }
//     }
// }

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error:{}, +error_id:{}", self.error, self.error_id)?;
        match &self.error_origin_position {
            None => {}
            Some(x) => {
                write!(f, " +Position:{x}")?;
            }
        }
        match &self.error_details {
            None => Ok(()),
            Some(x) => {
                write!(f, " +error_details:{x}")
            }
        }
    }
}
#[test]
fn test_display_error() {
    println!(
        "{}",
        AppError::new("eee".to_string()).with_error_origin_position("aaaa".to_string())
    );
}

impl AppError {
    pub fn new(error: String) -> Self {
        Self {
            error,
            error_id: Uuid::new_v4(),
            status: StatusCode::BAD_REQUEST,
            error_details: None,
            error_origin_position: None,
        }
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn with_details(mut self, details: Value) -> Self {
        self.error_details = Some(details);
        self
    }

    pub fn with_error_origin_position(mut self, position: String) -> Self {
        self.error_origin_position = Some(position);
        self
    }
}

impl<T: Error> From<T> for AppError {
    #[track_caller]
    fn from(value: T) -> Self {
        let caller_location = std::panic::Location::caller();
        let position = format!("{caller_location}");
        let app_error = AppError::new(format!("{value}",)).with_error_origin_position(position);
        tracing::debug!(
            "Position:{caller_location}; Error:{value}; Error ID:{};",
            app_error.error_id
        );
        app_error
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let mut res = axum::Json(self).into_response();
        *res.status_mut() = status;
        res
    }
}
