pub mod api;
pub mod api_doc;
pub mod auth;
pub mod db;
pub mod errors;

// #[cfg(feature = "postgres")]
// pub mod pg;
// pub mod handler;

#[macro_export]
macro_rules! impl_from {
    ($error:path) => {
        impl From<$error> for AuthError {
            fn from(_value: $error) -> Self {
                AuthError(AppError::from(_value))
            }
        }
    };
}

#[macro_export]
macro_rules! permission_layer {
    ($backend_type:ty, login_url = $login_url:expr, redirect_field = $redirect_field:expr, $($perm:expr),+ $(,)?) => {{
        use axum_login::AuthzBackend;

        async fn is_authorized(auth_session: axum_login::AuthSession<$backend_type>) -> bool {
            if let Some(ref user) = auth_session.user().await {
                if auth_session.backend().has_perm(user, crate::framework::auth::AuthPermission::Admin).await.unwrap_or(false) {
                    return true
                }
                let mut has_all_permissions = true;
                $(
                    has_all_permissions = has_all_permissions &&
                        auth_session.backend().has_perm(user, $perm.into()).await.unwrap_or(false);
                )+
                has_all_permissions
            } else {
                false
            }
        }

        axum_login::predicate_required!(
            is_authorized,
            login_url = $login_url,
            redirect_field = $redirect_field
        )
    }};

    ($backend_type:ty, login_url = $login_url:expr, $($perm:expr),+ $(,)?) => {
        crate::permission_layer!(
            $backend_type,
            login_url = $login_url,
            redirect_field = "next",
            $($perm),+
        )
    };

    ($backend_type:ty, $($perm:expr),+ $(,)?) => {{
        use axum_login::AuthzBackend;

        async fn is_authorized(auth_session: axum_login::AuthSession<$backend_type>) -> bool {
            if let Some(ref user) = auth_session.user().await {
                if auth_session.backend().has_perm(user, crate::framework::auth::AuthPermission::Admin).await.unwrap_or(false) {
                    return true
                }
                let mut has_all_permissions = true;
                $(
                    has_all_permissions = has_all_permissions &&
                        auth_session.backend().has_perm(user, $perm.into()).await.unwrap_or(false);
                )+
                has_all_permissions
            } else {
                false
            }
        }

        axum_login::predicate_required!(
            is_authorized,
            axum_login::axum::http::StatusCode::FORBIDDEN
        )
    }};
}
