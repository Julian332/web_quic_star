use crate::CURRENT_REQ;
use crate::db_model::user::User;
use alloy::rlp::Bytes;
use axum::body::{Body, to_bytes};
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use http::{HeaderValue, Method, StatusCode};
use std::mem;
use tracing::log::warn;
use uuid::fmt::Simple;

#[allow(unused)]
pub async fn save_req(mut request: Request, next: Next) -> Response {
    if request.method() == Method::GET {
        next.run(request).await
    } else {
        let path = request.uri().path().to_string();
        let type_flag = request
            .headers()
            .get(CONTENT_TYPE)
            .map(|content_type| {
                content_type == HeaderValue::from_static("application/x-www-form-urlencoded")
                    || content_type == HeaderValue::from_static("application/json")
            })
            .unwrap_or(false);
        let length_flag = request
            .headers()
            .get(CONTENT_LENGTH)
            .map(|x| x <= HeaderValue::from(usize::MAX))
            .unwrap_or(false);

        let req_body = if length_flag && type_flag {
            let body = mem::take(request.body_mut());
            let body_bytes = match to_bytes(body, usize::MAX).await {
                Ok(x) => x,
                Err(_) => {
                    warn!("req body exceeds it's content_length");
                    return StatusCode::BAD_REQUEST.into_response();
                }
            };

            let new_body = Body::from(body_bytes.clone());
            let _ = mem::replace(request.body_mut(), new_body);
            Some(body_bytes)
        } else {
            None
        };
        let response = next.run(request).await;
        let status_code = response.status();
        if CURRENT_REQ
            .try_with(move |x| {
                tokio::spawn(record(
                    x.user.clone(),
                    Some(x.req_id),
                    req_body,
                    path,
                    status_code,
                ))
            })
            .is_err()
        {
            eprintln!("save_req middleware's order is wrong")
        };
        response
    }
}

async fn record(
    user: Option<User>,
    req_id: Option<Simple>,
    req_body: Option<Bytes>,
    path: String,
    status_code: StatusCode,
) {
    println!("==={:?}", user);
    println!("{:?}", req_id);
    println!("{:?}", req_body);
    println!("{:?}", path);
    println!("{:?}", status_code);
}
