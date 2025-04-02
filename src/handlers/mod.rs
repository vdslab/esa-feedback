use crate::config::Config;
use crate::error::Result;
use crate::esa::models::WebhookPayload;
use regex::Regex;
use tracing::info;

pub mod weekly_report;

pub async fn route_webhook(webhook_payload: &WebhookPayload, config: &Config) -> Result<()> {
    // Skip processing if the post is a work in progress (wip)
    if webhook_payload.post.wip {
        info!("Skipping WIP post: {}", webhook_payload.post.number);
        return Ok(());
    }

    let post_path = webhook_payload
        .post
        .full_name
        .as_deref()
        .unwrap_or(&webhook_payload.post.name);
    info!("Routing post: {:?}", post_path);

    // Match patterns and call the appropriate handler
    if Regex::new(r"^週報/").unwrap().is_match(post_path) {
        info!("Matched weekly report pattern for post: {:?}", post_path);
        return weekly_report::handle(webhook_payload, config).await;
    } else {
        info!(
            "Post does not match any handler pattern, skipping: {:?}",
            post_path
        );
        return Ok(());
    }
}
