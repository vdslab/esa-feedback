use crate::error::Result;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub esa_access_token: String,
    pub esa_team_name: String,
    pub vertex_ai_project_id: String,
    pub vertex_ai_location: String,
    pub vertex_ai_model: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists
        let _ = dotenv();

        Ok(Self {
            esa_access_token: env::var("ESA_ACCESS_TOKEN")?,
            esa_team_name: env::var("ESA_TEAM_NAME")?,
            vertex_ai_project_id: env::var("VERTEX_AI_PROJECT_ID")?,
            vertex_ai_location: env::var("VERTEX_AI_LOCATION")?,
            vertex_ai_model: env::var("VERTEX_AI_MODEL")?,
        })
    }
}
