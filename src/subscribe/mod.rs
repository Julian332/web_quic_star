use crate::AppRes;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;
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
                sleep(Duration::new(2, 0)).await;
            }
        };
    }
}
