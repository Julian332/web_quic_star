use crate::db_model::req_record::NewReqRecord;
use crate::db_model::user::User;
use crate::framework::auth::ANONYMOUS_USER;
use crate::schema::req_records::dsl::req_records;
use crate::{AppRes, CURRENT_REQ, DB};
use alloy::rlp::Bytes;
use axum::body::{Body, to_bytes};
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use diesel_async::RunQueryDsl;
use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use http::{HeaderValue, Method, StatusCode};
use std::mem;
use std::str::FromStr;
use std::time::SystemTime;
use tracing::log::warn;
use uuid::fmt::Simple;

#[allow(unused)]
pub async fn save_req_to_db(mut request: Request, next: Next) -> Response {
    if request.method() == Method::GET {
        next.run(request).await
    } else {
        let path = request.uri().path().to_string();
        let sensitive_flag = !["/auth/login"].iter().any(|x| path.starts_with(x));

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
            .map(|x| {
                x.to_str()
                    .map(|x| usize::from_str(x).map(|x| true).unwrap_or(false))
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        let req_body = if length_flag && type_flag && sensitive_flag {
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
                    x.req_id,
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
    req_id: Simple,
    req_body: Option<Bytes>,
    path: String,
    status_code: StatusCode,
) -> AppRes<()> {
    let user_id = user.as_ref().map(|x| x.id).unwrap_or(ANONYMOUS_USER);
    let req_body = req_body.map(|x| {
        str::from_utf8(x.as_ref())
            .unwrap_or(" decoding utf8 err")
            .to_string()
    });
    diesel::insert_into(req_records)
        .values(NewReqRecord {
            username: user.map(|x| x.username),
            req_id: req_id.to_string(),
            req_body,
            path,
            status_code: status_code.to_string(),
            create_time: SystemTime::now().into(),
            create_by: user_id,
        })
        .execute(&mut DB.get().await?)
        .await?;
    Ok(())
}
