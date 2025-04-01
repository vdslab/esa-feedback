use crate::error::{Error, Result};
use crate::esa::models::WebhookPayload;
use serde_json::Value;

pub fn parse_webhook_payload(payload: Value) -> Result<WebhookPayload> {
    // Check if the payload has the expected structure
    if !payload.is_object() {
        return Err(Error::WebhookValidation(
            "Invalid webhook payload format".to_string(),
        ));
    }

    // Parse the payload
    let webhook_payload: WebhookPayload = serde_json::from_value(payload)
        .map_err(|e| Error::WebhookValidation(format!("Failed to parse webhook payload: {}", e)))?;

    // Validate the kind field
    match webhook_payload.kind.as_str() {
        "post_create" | "post_update" => {
            // These are the kinds we're interested in
            Ok(webhook_payload)
        }
        _ => Err(Error::WebhookValidation(format!(
            "Unsupported webhook kind: {}",
            webhook_payload.kind
        ))),
    }
}
