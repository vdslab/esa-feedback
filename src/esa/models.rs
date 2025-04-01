use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub screen_name: String,
    #[allow(dead_code)]
    pub icon: Option<UserIcon>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserIcon {
    #[allow(dead_code)]
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Post {
    pub number: i64,
    pub name: String,
    pub full_name: Option<String>,
    #[allow(dead_code)]
    pub wip: bool,
    pub body_md: String,
    #[allow(dead_code)]
    pub body_html: String,
    #[allow(dead_code)]
    pub category: Option<String>,
    #[allow(dead_code)]
    pub tags: Option<Vec<String>>,
    #[allow(dead_code)]
    pub url: String,
    #[allow(dead_code)]
    pub created_at: Option<String>,
    #[allow(dead_code)]
    pub updated_at: Option<String>,
    #[allow(dead_code)]
    pub message: Option<String>,
    #[allow(dead_code)]
    pub revision_number: Option<i64>,
    #[allow(dead_code)]
    pub created_by: Option<User>,
    #[allow(dead_code)]
    pub updated_by: Option<User>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Team {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookPayload {
    pub kind: String,
    pub team: Team,
    pub post: Post,
    #[allow(dead_code)]
    pub user: User,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommentPayload {
    pub body_md: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommentRequest {
    pub comment: CommentPayload,
}
