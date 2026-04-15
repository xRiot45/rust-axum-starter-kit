use tokio_cron_scheduler::{Job, JobScheduler};

/// Initializes the background job scheduler.
/// Add recurring jobs (cron-style) or one-shot tasks here.
pub async fn start_scheduler() -> anyhow::Result<JobScheduler> {
    let scheduler = JobScheduler::new().await?;

    // Example: run a cleanup job every night at midnight UTC
    scheduler
        .add(Job::new_async("0 0 0 * * *", |_uuid, _lock| {
            Box::pin(async move {
                tracing::info!("Running nightly cleanup job");
                // TODO: call CleanupService here
            })
        })?)
        .await?;

    scheduler.start().await?;
    tracing::info!("Background scheduler started");
    Ok(scheduler)
}
