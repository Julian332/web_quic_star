use crate::prelude::async_span;
use crate::web_middleware::ReqState;
use crate::{AppRes, CURRENT_REQ, DB};
use std::future::Future;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{debug, error};

pub async fn add_async_cron<R>(sched: &JobScheduler, cron: &str, task: fn() -> R)
where
    R: Future<Output = AppRes<()>> + Sized + Send + 'static,
{
    sched
        .add(
            Job::new_async(cron, move |_uuid, _l| {
                Box::pin(async move {
                    match CURRENT_REQ
                        .scope(ReqState::default(), async_span(task()))
                        .await
                    {
                        Ok(_) => {
                            debug!("cron task succeed");
                        }
                        Err(e) => {
                            error!("cron task failed: {}", e);
                        }
                    };
                })
            })
            .expect("cannot create job"),
        )
        .await
        .expect("cannot join job");
}
pub async fn example() -> AppRes<()> {
    let _connection = DB.get().await?;
    debug!("example connection");
    Ok(())
}

pub async fn set_scheduler() {
    let sched = JobScheduler::new()
        .await
        .expect("cannot create jobs scheduler");
    #[cfg(feature = "dev")]
    add_async_cron(&sched, "1/5 * * * * *", example).await;

    sched.start().await.expect("cannot start jobs scheduler");
}
