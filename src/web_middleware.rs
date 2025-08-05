use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use crate::CURRENT_REQ_HEADER;

pub async fn save_req_to_task_local(
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request,
    next: Next,
) -> Response {
    let response = CURRENT_REQ_HEADER
        .scope(
            request.headers().clone(),
            async move { next.run(request).await },
        )
        .await;
    response
}