use aide::OperationIo;
use axum::{http::StatusCode, response::IntoResponse};
use derive_more::{Display, Error};
use schemars::json_schema;
use serde_json::Value;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use tracing::warn;
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
                    schemars::_private::insert_object_property(&mut schema, "error", false, {
                        let mut schema = json_schema!({
                        "type": "string",
                            });
                        schemars::_private::insert_metadata_property_if_nonempty(
                            &mut schema,
                            "title",
                            {
                                const TITLE: &str = schemars::_private::get_title_and_description(
                                    ::core::concat!("An error message."),
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
                                        "An error message."
                                    ))
                                    .1;
                                DESCRIPTION
                            },
                        );
                        schema
                    });
                }
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
                                    const TITLE: &str =
                                        schemars::_private::get_title_and_description(
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
                                        schemars::_private::get_title_and_description(
                                            ::core::concat!("A unique error ID."),
                                        )
                                        .1;
                                    DESCRIPTION
                                },
                            );
                            schema
                        },
                    );
                }
                {
                    schemars::_private::insert_object_property(&mut schema, "status", false, {
                        generator.subschema_for::<u16>()
                    });
                }
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "error_details",
                        generator.contract().is_deserialize()
                            && <Option<Value> as schemars::JsonSchema>::_schemars_private_is_option(
                            ),
                        {
                            let mut schema = generator.subschema_for::<Option<Value>>();
                            schemars::_private::insert_metadata_property_if_nonempty(
                                &mut schema,
                                "title",
                                {
                                    const TITLE: &str =
                                        schemars::_private::get_title_and_description(
                                            ::core::concat!("Optional Additional error details."),
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
                                        schemars::_private::get_title_and_description(
                                            ::core::concat!("Optional Additional error details."),
                                        )
                                        .1;
                                    DESCRIPTION
                                },
                            );
                            schema
                        },
                    );
                }
                {
                    schemars::_private::insert_object_property(&mut schema, "error_origin_position", generator.contract().is_deserialize() && <Option<String> as schemars::JsonSchema>::_schemars_private_is_option(), { generator.subschema_for::<Option<String>>() });
                }
                schemars::_private::insert_metadata_property_if_nonempty(&mut schema, "title", {
                    const TITLE: &str = schemars::_private::get_title_and_description(
                        ::core::concat!("A default error response for most API errors."),
                    )
                    .0;
                    TITLE
                });
                schemars::_private::insert_metadata_property_if_nonempty(
                    &mut schema,
                    "description",
                    {
                        const DESCRIPTION: &str = schemars::_private::get_title_and_description(
                            ::core::concat!("A default error response for most API errors."),
                        )
                        .1;
                        DESCRIPTION
                    },
                );
                schema
            }
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
    // println!("{}", error);
    // println!("{:?}", serde_json::to_string(&error));
    use alloy::rpc::types::BlockError;

    let x: AppError = BlockError::InvalidSignature.into();
    println!("{:?}", x);
}

impl AppError {
    pub fn new(error: &str) -> Self {
        Self {
            error: anyhow::anyhow!("{error}"),
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
        warn!("request failed :{self:?}");
        let status = self.status.clone();
        let mut res = axum::Json(self).into_response();
        *res.status_mut() = status;
        res
    }
}
