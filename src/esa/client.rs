use crate::error::{Error, Result};
use crate::esa::models::{CommentPayload, CommentRequest};
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

pub struct EsaClient {
    client: Client,
    team_name: String,
    base_url: String,
}

impl EsaClient {
    pub fn new(team_name: &str, access_token: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token))
                .map_err(|e| Error::EsaApi(e.to_string()))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| Error::EsaApi(e.to_string()))?;

        Ok(Self {
            client,
            team_name: team_name.to_string(),
            base_url: "https://api.esa.io/v1".to_string(),
        })
    }

    pub async fn create_comment(&self, post_number: i64, body: &str) -> Result<()> {
        let url = format!(
            "{}/teams/{}/posts/{}/comments",
            self.base_url, self.team_name, post_number
        );

        let payload = CommentRequest {
            comment: CommentPayload {
                body_md: body.to_string(),
                user: Some("esa_bot".to_string()),
            },
        };

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| Error::EsaApi(format!("Failed to create comment: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::EsaApi(format!(
                "Failed to create comment. Status: {}, Error: {}",
                status, error_text
            )));
        }

        Ok(())
    }
}
