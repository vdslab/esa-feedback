use axum_test::TestServer;
use esa_feedback::{config::Config, server::create_app};
use serde_json::json;

// Helper function to create a test config
fn create_test_config() -> Config {
    Config::from_env().expect("Failed to load config from environment")
}

// 記事の本文
const POST_CONTENT: &str = r#"# 目標トロフィー

<!-- トロフィーリスト（ https://vdslab.esa.io/posts/5499 ）から項目をコピーしてください。 -->
- [ ] 緑色コーダーになった（達成日：〇〇〇〇年〇〇月〇〇日）
- [ ] 技術記事を1本公開した（達成日：〇〇〇〇年〇〇月〇〇
- [ ] TOEICスコア500を取った（達成日：〇〇〇〇年〇〇月〇〇日）
- [ ] Webアプリを1個公開した（達成日：〇〇〇〇年〇〇月〇〇)Webアプリを1個公開した（達成日：〇〇〇〇年〇〇月〇〇)
- [ ] 情報処理安全確保支援士試験に合格した（達成日：〇〇〇〇年〇〇月〇〇日）


# 今週達成したトロフィー

<!-- トロフィーリスト（ https://vdslab.esa.io/posts/5499 ）から項目をコピーしてください。 -->

なし

# 今週やったこと

<!-- 前回の「来週やること」から項目をコピーして、不足があれば追記してください。 -->
<!-- 内容は具体的に書きましょう。（×：課題、〇：オブジェクト指向の課題：抽象クラスの演習） -->

- [ ] コンテスト出たい（今週は出る）
- [x] チームMTG
- [ ] チームの発表
- [x] チームアプリの開発
- [x] バイト
- [x] 発展プロの課題
- [ ] コンピューティングの追加課題
- [x] アルゴリズムのテスト

# 今週学んだこと・わかったこと

<!-- 専門知識に関する学びを含めましょう。 -->

- 面接でよく聞かれるのは、チーム開発経験らしい。

# 来週やること

<!-- トロフィー達成に向けて取り組むことを含めましょう。 -->

- [ ] 楽譜
- [ ] チーム発表
- [ ] データべのアプリをブラッシュアップしたい。
- [ ] コンテストに出る。

# 今週見た面白いビジュアライゼーション

- 

# 所感

- 疲労困憊"#;

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
