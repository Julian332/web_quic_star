use crate::CURRENT_REQ;
use aide::OperationIo;
use anyhow::anyhow;
use axum::response::IntoResponse;
use derive_more::{Display, Error};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;
pub trait IntoResult<T, E> {
    fn into_result(self) -> Result<T, E>;
}

impl<T> IntoResult<T, NoneError> for Option<T> {
    fn into_result(self) -> Result<T, NoneError> {
        self.ok_or(NoneError)
    }
}
#[allow(unused)]
#[derive(Debug, Display, Error)]
pub struct NoneError;
/// A default error response for most API errors.
#[derive(Debug, OperationIo)]
pub struct AppError {
    error_id: Uuid,
    error: anyhow::Error,
}

impl serde::Serialize for AppError {
    fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let err = format!("{}", self.error);
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "AppError", 2)?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "error_id",
            &self.error_id,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "error", &err)?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}

impl schemars::JsonSchema for AppError {
    fn inline_schema() -> bool {
        false
    }
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("AppError")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed(::core::concat!(
            ::core::module_path!(),
            "::",
            "AppError"
        ))
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        {
            let mut schema = schemars::json_schema!({"type" : "object" , });
            {
                schemars::_private::insert_object_property(
                    &mut schema,
                    "error_id",
                    generator.contract().is_deserialize()
                        && <Uuid as schemars::JsonSchema>::_schemars_private_is_option(),
                    {
                        let mut schema = generator.subschema_for::<Uuid>();
                        schemars::_private::insert_metadata_property_if_nonempty(
                            &mut schema,
                            "title",
                            {
                                const TITLE: &str = schemars::_private::get_title_and_description(
                                    ::core::concat!("A unique error ID."),
                                )
                                .0;
                                TITLE
                            },
                        );
                        schemars::_private::insert_metadata_property_if_nonempty(
                            &mut schema,
                            "description",
                            {
                                const DESCRIPTION: &str =
                                    schemars::_private::get_title_and_description(::core::concat!(
                                        "A unique error ID."
                                    ))
                                    .1;
                                DESCRIPTION
                            },
                        );
                        schema
                    },
                );
                schemars::_private::insert_object_property(
                    &mut schema,
                    "error",
                    generator.contract().is_deserialize()
                        && <String as schemars::JsonSchema>::_schemars_private_is_option(),
                    {
                        let mut schema = generator.subschema_for::<String>();
                        schemars::_private::insert_metadata_property_if_nonempty(
                            &mut schema,
                            "title",
                            {
                                const TITLE: &str = schemars::_private::get_title_and_description(
                                    ::core::concat!("error reason"),
                                )
                                .0;
                                TITLE
                            },
                        );
                        schemars::_private::insert_metadata_property_if_nonempty(
                            &mut schema,
                            "description",
                            {
                                const DESCRIPTION: &str =
                                    schemars::_private::get_title_and_description(::core::concat!(
                                        "error reason"
                                    ))
                                    .1;
                                DESCRIPTION
                            },
                        );
                        schema
                    },
                );
            }
            schemars::_private::insert_metadata_property_if_nonempty(&mut schema, "title", {
                const TITLE: &str = schemars::_private::get_title_and_description(::core::concat!(
                    "A default error response for most API errors."
                ))
                .0;
                TITLE
            });
            schemars::_private::insert_metadata_property_if_nonempty(&mut schema, "description", {
                const DESCRIPTION: &str = schemars::_private::get_title_and_description(
                    ::core::concat!("A default error response for most API errors."),
                )
                .1;
                DESCRIPTION
            });
            schema
        }
    }
}

impl<T: Error + Send + Sync + 'static> From<T> for AppError {
    fn from(value: T) -> Self {
        let value = anyhow!(value);
        let uuid = CURRENT_REQ
            .try_with(|x| x.req_id.into_uuid())
            .unwrap_or_else(|_| Uuid::now_v7());
        tracing::debug!("Error:{value:?}; Error ID:{uuid};");
        Self {
            error: value,
            error_id: uuid,
        }
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

impl AppError {
    pub fn new(error: &str) -> Self {
        let uuid = CURRENT_REQ
            .try_with(|x| x.req_id.into_uuid())
            .unwrap_or_else(|_| Uuid::now_v7());
        Self {
            error: anyhow::anyhow!("{error}"),
            error_id: uuid,
        }
    }
}
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use schemars::{JsonSchema, SchemaGenerator};
    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn async_test1() {}
    #[test]
    fn sync_test1() {
        let error = AppError::new("test");
        println!("{}", error);
        println!("{}", serde_json::to_string(&error).unwrap());
        let schema = AppError::json_schema(&mut SchemaGenerator::default());
        println!("{}", serde_json::to_string(&schema).unwrap());
        println!("{:?}", error);
    }
}
