use std::sync::Arc;

use aide::{
    axum::{ApiRouter, IntoApiResponse},
    openapi::OpenApi,
};
use axum::{response::IntoResponse, Extension};

use axum::Json;

#[cfg(not(feature = "dev"))]
pub fn docs_routes() -> ApiRouter {
    // We infer the return types for these routes
    // as an example.
    //
    // As a result, the `serve_redoc` route will
    // have the `text/html` content-type correctly set
    // with a 200 status.
    aide::generate::infer_responses(true);

    let router: ApiRouter = ApiRouter::new();
    // .with_state(state);

    // Afterwards we disable response inference because
    // it might be incorrect for other routes.
    aide::generate::infer_responses(false);
    router
}
#[cfg(feature = "dev")]
pub fn docs_routes() -> ApiRouter {
    // We infer the return types for these routes
    // as an example.
    //
    // As a result, the `serve_redoc` route will
    // have the `text/html` content-type correctly set
    // with a 200 status.
    aide::generate::infer_responses(true);

    let router: ApiRouter = ApiRouter::new()
        .api_route_with(
            "/",
            aide::axum::routing::get_with(
                aide::swagger::Swagger::new("/docs/private/api.json")
                    .with_title("web_quic_star")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            |p| p.security_requirement("ApiKey"),
        )
        .api_route_with(
            "/pretty_doc",
            aide::axum::routing::get_with(
                aide::scalar::Scalar::new("/docs/private/api.json")
                    .with_title("web_quic_star")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            |p| p.security_requirement("ApiKey"),
        )
        // .api_route_with(
        //     "/redoc",
        //     aide::axum::routing::get_with(
        //         aide::redoc::Redoc::new("/docs/private/api.json")
        //             .with_title("web_quic_star")
        //             .axum_handler(),
        //         |op| op.description("This documentation page."),
        //     ),
        //     |p| p.security_requirement("ApiKey"),
        // )
        .route("/private/api.json", aide::axum::routing::get(serve_docs));
    // .with_state(state);

    // Afterwards we disable response inference because
    // it might be incorrect for other routes.
    aide::generate::infer_responses(false);
    router
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}
