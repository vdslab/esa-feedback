use crate::config::Config;
use crate::error::{Error, Result};
use crate::esa::client::EsaClient;
use crate::esa::models::WebhookPayload;
use crate::vertex_ai::client::VertexAiClient;
use std::fs;
use std::path::Path;

pub async fn handle(webhook_payload: &WebhookPayload, config: &Config) -> Result<()> {
    // Load prompt from the same directory
    let prompt_path = Path::new(file!()).parent().unwrap().join("prompt.txt");
    let prompt = fs::read_to_string(prompt_path).map_err(|e| Error::Io(e))?;

    // Initialize Vertex AI client
    let vertex_client = VertexAiClient::new(
        &config.vertex_ai_project_id,
        &config.vertex_ai_location,
        &config.vertex_ai_model,
    )?;

    // Generate feedback
    let feedback = vertex_client
        .generate_feedback(&webhook_payload.post.body_md, &prompt)
        .await?;

    // Initialize esa.io client
    let esa_client = EsaClient::new(&config.esa_team_name, &config.esa_access_token)?;

    // Post feedback as a comment
    esa_client
        .create_comment(webhook_payload.post.number, &feedback)
        .await?;

    Ok(())
}
