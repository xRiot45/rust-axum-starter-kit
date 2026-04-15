use uuid::Uuid;

/// Async audit logging job.
pub async fn log_audit_event(user_id: Uuid, action: &str, resource: &str) -> anyhow::Result<()> {
    tracing::info!(user_id = %user_id, action, resource, "Audit event (stub)");
    // TODO: persist to audit_logs table or a message queue
    Ok(())
}
