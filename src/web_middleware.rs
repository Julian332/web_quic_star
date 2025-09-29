use crate::CURRENT_REQ;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use http::HeaderMap;
use uuid::Uuid;

pub async fn save_req_to_task_local(
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    let req_id = Uuid::now_v7();
    let response = CURRENT_REQ
        .scope((req_id, headers), async move { next.run(request).await })
        .await;
    response
}
