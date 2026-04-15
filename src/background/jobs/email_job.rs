/// Email sending job.
/// Use lettre + SMTP or a transactional email provider (SendGrid, Resend, etc.)
pub async fn send_welcome_email(to: &str, name: &str) -> anyhow::Result<()> {
    tracing::info!(to, name, "Sending welcome email (stub)");
    // TODO: integrate with lettre or an external email API
    Ok(())
}
