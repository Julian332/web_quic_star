use aide::OperationIo;
use axum::{http::StatusCode, response::IntoResponse};
use derive_more::{Display, Error};
use serde_json::Value;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;
/// A default error response for most API errors.
#[derive(Debug, OperationIo)]
pub struct AppError {
    /// An error message.
    error: anyhow::Error,
    /// A unique error ID.
    error_id: Uuid,
    status: StatusCode,
    /// Optional Additional error details.
    error_details: Option<Value>,
    error_origin_position: Option<String>,
}

const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for AppError {
        fn schema_name() -> std::string::String {
            "AppError".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed(std::concat!(std::module_path!(), "::", "AppError"))
        }
        fn json_schema(
            generator: &mut schemars::r#gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            schemars::_private::metadata::add_description(
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Object.into()),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    {
                        schemars::_private::insert_object_property::<String>(
                            object_validation,
                            "error",
                            false,
                            false,
                            schemars::_private::metadata::add_description(
                                generator.subschema_for::<String>(),
                                "An error message.",
                            ),
                        );
                    }
                    {
                        schemars::_private::insert_object_property::<Uuid>(
                            object_validation,
                            "error_id",
                            false,
                            false,
                            schemars::_private::metadata::add_description(
                                generator.subschema_for::<Uuid>(),
                                "A unique error ID.",
                            ),
                        );
                    }
                    {
                        schemars::_private::insert_object_property::<Option<Value>>(
                            object_validation,
                            "error_details",
                            false,
                            false,
                            schemars::_private::metadata::add_description(
                                generator.subschema_for::<Option<Value>>(),
                                "Optional Additional error details.",
                            ),
                        );
                    }
                    {
                        schemars::_private::insert_object_property::<Option<String>>(
                            object_validation,
                            "error_origin_position",
                            false,
                            false,
                            generator.subschema_for::<Option<String>>(),
                        );
                    }
                    schemars::schema::Schema::Object(schema_object)
                },
                "A default error response for most API errors.",
            )
        }
    }
};

#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for AppError {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "AppError",
                false as usize
                    + 1
                    + 1
                    + if Option::is_none(&self.error_details) {
                        0
                    } else {
                        1
                    }
                    + if Option::is_none(&self.error_origin_position) {
                        0
                    } else {
                        1
                    },
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "error",
                &self.error.to_string(),
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "error_id",
                &self.error_id,
            )?;
            if !Option::is_none(&self.error_details) {
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "error_details",
                    &self.error_details,
                )?;
            } else {
                _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "error_details")?;
            }
            if !Option::is_none(&self.error_origin_position) {
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "error_origin_position",
                    &self.error_origin_position,
                )?;
            } else {
                _serde::ser::SerializeStruct::skip_field(
                    &mut __serde_state,
                    "error_origin_position",
                )?;
            }
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[allow(unused)]
#[derive(Debug, Display, Error)]
pub struct NoneError;
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
    let error =
        AppError::new("eee").with_error_origin_position("error_origin_position".to_string());
    println!("{:?}", error);
    println!("{}", error);
    println!("{:?}", serde_json::to_string(&error));
}

impl AppError {
    pub fn new(error: &'static str) -> Self {
        Self {
            error: anyhow::anyhow!(error),
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

impl<T: Error + Send + Sync + 'static> From<T> for AppError {
    #[track_caller]
    fn from(value: T) -> Self {
        let caller_location = std::panic::Location::caller();
        let position = format!("{caller_location}");
        let uuid = Uuid::new_v4();
        tracing::debug!(
            "Position:{caller_location}; Error:{value}; Error ID:{};",
            uuid
        );
        let app_error = Self {
            error: value.into(),
            error_id: uuid,
            status: StatusCode::BAD_REQUEST,
            error_details: None,
            error_origin_position: Some(position),
        };

        app_error
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status.clone();
        let mut res = axum::Json(self).into_response();
        *res.status_mut() = status;
        res
    }
}
