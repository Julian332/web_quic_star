use crate::CURRENT_REQ;
use crate::web_middleware::ReqState;
use axum_login::AuthUser;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::task::JoinHandle;
use tracing::error;

pub async fn sleep_ms(ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await
}
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let req_state = CURRENT_REQ
        .try_with(|x| {
            let mut spawn_id = x.spawn_id.clone();
            spawn_id.push(x.spawn_count.load(Ordering::SeqCst));
            x.spawn_count.fetch_add(1, Ordering::SeqCst);
            ReqState {
                req_id: x.req_id,
                user: x.user.clone(),
                spawn_id,
                spawn_count: Default::default(),
            }
        })
        .unwrap_or_default();
    tokio::spawn(async move { CURRENT_REQ.scope(req_state, async_span(future)).await })
}
#[tracing::instrument(
    name = "spawn",
    level = "info",
    skip_all,
    fields(user_id, req_id, spawn_id)
)]
pub async fn async_span<F>(future: F) -> F::Output
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    if CURRENT_REQ
        .try_with(|req_state| {
            tracing::Span::current().record("req_id", req_state.req_id.to_string());

            if let Some(user) = &req_state.user {
                tracing::Span::current().record("user_id", user.id().to_string());
            }
            tracing::Span::current().record("spawn_id", format!("{:?}", req_state.spawn_id));
        })
        .is_err()
    {
        error!("CURRENT_REQ not set")
    };
    future.await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tracing_test::traced_test]
    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn async_test1() {
        info!("0");

        spawn(async {
            info!("1");
            println!("1");
            spawn(async {
                info!("3");
            });
            spawn(async {
                info!("2");
                spawn(async {
                    info!("0");
                });
                spawn(async {
                    info!("1");
                });
            });
        });
        sleep_ms(5000).await;
    }
    #[test]
    fn sync_test1() {}
}
