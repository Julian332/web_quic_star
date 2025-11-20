use crate::AppRes;
use crate::prelude::sleep_ms;
use std::future::Future;
use tracing::error;

pub async fn subscribe_with_retry<F>(func: fn() -> F) -> AppRes<()>
where
    F: Future<Output = AppRes<()>> + Sized,
{
    loop {
        match func().await {
            Ok(_) => {}
            Err(e) => {
                error!(?e, " Will retry subscribe");
                sleep_ms(2000).await;
            }
        };
    }
}
