pub mod api;
pub mod api_doc;
pub mod auth;
pub mod db;
pub mod errors;
pub mod handler;

#[macro_export]
macro_rules! impl_from {
    ($error:path) => {
        impl From<$error> for AuthError {
            fn from(_value: $error) -> Self {
                AuthError(AppError::new("auth error"))
            }
        }
    };
}
