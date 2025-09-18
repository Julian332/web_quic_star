use crate::CURRENT_REQ_HEADER;
use aide::OperationIo;
use anyhow::anyhow;
use axum::response::IntoResponse;
use derive_more::{Display, Error};
use schemars::json_schema;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use tracing::info;
use uuid::Uuid;

#[allow(unused)]
#[derive(Debug, Display, Error)]
pub struct NoneError;
/// A default error response for most API errors.
#[derive(Debug, OperationIo)]
pub struct AppError {
    /// A unique error ID.
    error_id: Uuid,
    error: anyhow::Error,
}
impl<T: Error + Send + Sync + 'static> From<T> for AppError {
    fn from(value: T) -> Self {
        let value = anyhow!(value);
        let uuid = Uuid::new_v4();
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::absolute_paths, )]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    _serde::__require_serde_not_serde_core!();   #[automatically_derived]
    impl _serde::Serialize for AppError {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::__private225::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "AppError", false as usize + 1 + 1)?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "error_id", &self.error_id)?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "error", &self.error.to_string())?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
