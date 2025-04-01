use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub name: String,
    pub screen_name: String,
    pub icon: Option<UserIcon>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserIcon {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Post {
    pub number: i64,
    pub name: String,
    pub full_name: Option<String>,
    pub wip: bool,
    pub body_md: String,
    pub body_html: String,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub url: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub message: Option<String>,
    pub revision_number: Option<i64>,
    pub created_by: Option<User>,
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
    pub user: User,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommentPayload {
    pub body_md: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommentRequest {
    pub comment: CommentPayload,
}
