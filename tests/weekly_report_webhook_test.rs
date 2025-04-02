use axum_test::TestServer;
use esa_feedback::{config::Config, server::create_app};
use serde_json::json;

// Helper function to create a test config
fn create_test_config() -> Config {
    Config::from_env().expect("Failed to load config from environment")
}

// 記事の本文
const POST_CONTENT: &str = include_str!("test_data/weekly_report_content.txt");

/// Creates a post_create webhook payload
fn create_post_create_payload(wip: bool) -> serde_json::Value {
    json!({
        "kind": "post_create",
        "team": {
            "name": "vdslab"
        },
        "post": {
            "name": "週報/2025/04/02/週報",
            "body_md": POST_CONTENT,
            "body_html": "<p>HTML content</p>",
            "message": "Create post.",
            "wip": wip,
            "number": 6602,
            "url": "https://vdslab.esa.io/posts/6602"
        },
        "user": {
            "icon": {
                "url": "https://img.esa.io/uploads/production/users/1/icon/thumb_m_example.png"
            },
            "name": "Test User",
            "screen_name": "testuser"
        }
    })
}

/// Creates a post_update webhook payload
fn create_post_update_payload(wip: bool) -> serde_json::Value {
    json!({
        "kind": "post_update",
        "team": {
            "name": "vdslab"
        },
        "post": {
            "name": "週報/2025/04/02/週報",
            "body_md": POST_CONTENT,
            "body_html": "<p>HTML content</p>",
            "message": "Update post.",
            "wip": wip,
            "number": 6602,
            "url": "https://vdslab.esa.io/posts/6602",
            "diff_url": "https://vdslab.esa.io/posts/6602/revisions/2"
        },
        "user": {
            "icon": {
                "url": "https://img.esa.io/uploads/production/users/1/icon/thumb_m_example.png"
            },
            "name": "Test User",
            "screen_name": "testuser"
        }
    })
}

#[tokio::test]
async fn test_webhook_post_create() {
    // Create a test app
    let app = create_app(create_test_config());

    // Create a test server
    let server = TestServer::new(app.into_make_service()).unwrap();

    // Create a post_create webhook payload
    let payload = create_post_create_payload(false);

    // Send the request to the app
    let response = server
        .post("/webhook")
        .json(&payload)
        .add_header("Content-Type", "application/json")
        .add_header("User-Agent", "esa-Hookshot/v1")
        .add_header("X-Esa-Delivery", "12345")
        .await;

    // Check the response
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_webhook_post_update() {
    // Create a test app
    let app = create_app(create_test_config());

    // Create a test server
    let server = TestServer::new(app.into_make_service()).unwrap();

    // Create a post_update webhook payload
    let payload = create_post_update_payload(false);

    // Send the request to the app
    let response = server
        .post("/webhook")
        .json(&payload)
        .add_header("Content-Type", "application/json")
        .add_header("User-Agent", "esa-Hookshot/v1")
        .add_header("X-Esa-Delivery", "12346")
        .await;

    // Check the response
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_webhook_wip_post() {
    // Create a test app
    let app = create_app(create_test_config());

    // Create a test server
    let server = TestServer::new(app.into_make_service()).unwrap();

    // Create a post_create webhook payload with wip=true
    let payload = create_post_create_payload(true);

    // Send the request to the app
    let response = server
        .post("/webhook")
        .json(&payload)
        .add_header("Content-Type", "application/json")
        .add_header("User-Agent", "esa-Hookshot/v1")
        .add_header("X-Esa-Delivery", "12347")
        .await;

    // Check the response - should still return OK even though processing is skipped
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_webhook_wip_post_update() {
    // Create a test app
    let app = create_app(create_test_config());

    // Create a test server
    let server = TestServer::new(app.into_make_service()).unwrap();

    // Create a post_update webhook payload with wip=true
    let payload = create_post_update_payload(true);

    // Send the request to the app
    let response = server
        .post("/webhook")
        .json(&payload)
        .add_header("Content-Type", "application/json")
        .add_header("User-Agent", "esa-Hookshot/v1")
        .add_header("X-Esa-Delivery", "12348")
        .await;

    // Check the response - should still return OK even though processing is skipped
    assert_eq!(response.status_code(), 200);
}
