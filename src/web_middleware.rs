use crate::CURRENT_REQ;
use crate::db_model::user::User;
use crate::framework::auth::AuthBackend;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum_login::AuthUser;
use http::Method;
use std::sync::atomic::AtomicUsize;
use tokio::time::Instant;
use tracing::{error, info};
use uuid::Uuid;
use uuid::fmt::Simple;

pub struct ReqState {
    pub req_id: Simple,
    pub user: Option<User>,
    pub spawn_id: Vec<usize>,
    pub spawn_count: AtomicUsize,
}

impl Default for ReqState {
    fn default() -> Self {
        let req_id = Uuid::now_v7().simple();
        Self {
            req_id,
            user: None,
            spawn_id: vec![],
            spawn_count: Default::default(),
        }
    }
}

#[tracing::instrument(name = "", level = "info", skip_all, fields(user_id, req_id))]
pub async fn save_req_to_task_local(
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    auth_session: axum_login::AuthSession<AuthBackend>,
    request: Request,
    next: Next,
) -> Response {
    let req_id = Uuid::now_v7().simple();
    let user = auth_session.user().await;
    tracing::Span::current().record("req_id", req_id.to_string());

    if let Some(user) = &user {
        tracing::Span::current().record("user_id", user.id().to_string());
    }
    CURRENT_REQ
        .scope(
            ReqState {
                req_id,
                user,
                spawn_id: vec![],
                spawn_count: Default::default(),
            },
            async move { next.run(request).await },
        )
        .await
}

pub async fn req_log(request: Request, next: Next) -> Response {
    let instant = Instant::now();
    let method = request.method().to_string();
    let uri = request.uri().to_string();
    let response = next.run(request).await;
    let duration = instant.elapsed();
    info!("req:{method} {uri} spent {duration:?}");
    response
}

pub async fn continue_when_drop_req(request: Request, next: Next) -> Response {
    if request.method() == Method::GET {
        next.run(request).await
    } else {
        let result = tokio::spawn(async move { next.run(request).await }).await;
        result.unwrap_or_else(|e| {
            error!("continue_when_drop_req error: {e}");
            Response::default()
        })
    }
}
