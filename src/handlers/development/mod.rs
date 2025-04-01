use crate::config::Config;
use crate::error::Result;
use crate::esa::client::EsaClient;
use crate::esa::models::WebhookPayload;
use crate::vertex_ai::client::VertexAiClient;

pub async fn handle(webhook_payload: &WebhookPayload, config: &Config) -> Result<()> {
    // プロンプトをinclude_str!マクロで読み込む
    let prompt = include_str!("prompt.txt");
    let content = &webhook_payload.post.body_md;

    // Initialize Vertex AI client
    let vertex_client = VertexAiClient::new(
        &config.vertex_ai_project_id,
        &config.vertex_ai_location,
        &config.vertex_ai_model,
    )?;

    // Generate feedback
    let feedback = vertex_client.generate_feedback(content, prompt).await?;

    // Initialize esa.io client
    let esa_client = EsaClient::new(&config.esa_team_name, &config.esa_access_token)?;

    // Post feedback as a comment
    esa_client
        .create_comment(webhook_payload.post.number, &feedback)
        .await?;

    Ok(())
}
