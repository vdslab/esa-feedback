use crate::config::Config;
use crate::error::{Error, Result};
use crate::esa::models::WebhookPayload;
use crate::handlers::route_webhook;
use serde_json::Value;
use tracing::info;

pub async fn handle_webhook(payload: Value, config: &Config) -> Result<()> {
    // Parse the webhook payload
    let webhook_payload = crate::esa::webhook::parse_webhook_payload(payload)?;

    // Log the received webhook
    info!(
        "Received webhook: kind={}, team={}, post={}",
        webhook_payload.kind, webhook_payload.team.name, webhook_payload.post.name
    );

    // Route the webhook to the appropriate handler
    route_webhook(&webhook_payload, config).await
}
