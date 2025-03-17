use crate::api_router::docs::docs_routes;
use crate::framework::errors::AppError;
use aide::axum::ApiRouter;
use aide::openapi::OpenApi;
use aide::transform::{TransformOpenApi, TransformOperation};
use axum::http::StatusCode;
use axum::http::Uri;
use axum::{Extension, Router};
use schemars::JsonSchema;
use serde::Serialize;
use std::sync::Arc;

pub fn default_resp_docs<Resp: JsonSchema + Serialize>(
    op: TransformOperation,
) -> TransformOperation {
    op.description("default_docs")
}

// pub fn default_resp_docs_with_exam<Resp: JsonSchema + Serialize>(
//     op: TransformOperation,
// ) -> TransformOperation {
//     op.description("default_docs").response::<200, Json<Resp>>()
// }

pub fn empty_resp_docs(op: TransformOperation) -> TransformOperation {
    op.description("default_docs")
    // .response::<200,Json<Resp>>()
}
pub fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("RsWebQuicStar")
        .summary("RsWebQuicStar")
        .description(include_str!("api-doc.md"))
        // .tag(Tag {
        //     name: "tod".into(),
        //     description: Some("Tod Management".into()),
        //     ..Default::default()
        // })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Cookie,
                name: "id".into(),
                description: Some("HttpOnly; SameSite=Strict; Path=/; ".into()),
                extensions: Default::default(),
            },
        )
        .default_response_with::<axum::Json<AppError>, _>(|res| {
            res.example(
                AppError::new("some error happened".to_string())
                    .with_status(StatusCode::IM_A_TEAPOT),
            )
        })
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
pub async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}

pub fn set_api_doc(app: ApiRouter) -> Router {
    let mut api = OpenApi::default();
    app.nest_api_service("/docs", docs_routes())
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)))
}
