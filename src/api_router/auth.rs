use crate::db_model::user::User;
use crate::framework::api_doc::default_resp_docs;
use crate::framework::auth::{AuthBackend, Credentials};
use aide::axum::routing::post_with;
use aide::axum::{ApiRouter, IntoApiResponse};
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum_login::AuthSession;

pub async fn login(
    auth_session: AuthSession<AuthBackend>,
    Json(creds): Json<Credentials>,
) -> impl IntoApiResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Some(ref next) = creds.next {
        Redirect::to(next).into_response()
    } else {
        StatusCode::OK.into_response()
    }
}

pub fn router() -> ApiRouter<()> {
    ApiRouter::new().api_route("/login", post_with(login, default_resp_docs::<User>))
}
