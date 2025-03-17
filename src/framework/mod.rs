pub mod api;
pub mod api_doc;
pub mod auth;
pub mod db;
pub mod errors;

#[macro_export]
macro_rules! impl_from {
    ($error:path) => {
        impl From<$error> for AuthError {
            fn from(value: $error) -> Self {
                AuthError(AppError::new(format!("error:::::::: {}", value)))
            }
        }
    };
}
