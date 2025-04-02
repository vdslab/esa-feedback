use esa_feedback::esa::webhook::parse_webhook_payload;
use serde_json::json;

#[test]
fn test_parse_webhook_payload_post_create() {
    // テストデータ
    let payload = json!({
        "kind": "post_create",
        "team": {
            "name": "esa"
        },
        "post": {
            "name": "foo/bar/たいとる #tag1 #tag2",
            "body_md": "ほんぶん",
            "body_html": "<p>ほんぶん</p>\n",
            "message": "Create post.",
            "wip": false,
            "number": 1253,
            "url": "https://example.esa.io/posts/1253"
        },
        "user": {
            "icon": {
                "url": "https://img.esa.io/uploads/production/users/1/icon/402685a258cf2a33c1d6c13a89adec92.png",
                "thumb_s": {
                    "url": "https://img.esa.io/uploads/production/users/1/icon/thumb_s_402685a258cf2a33c1d6c13a89adec92.png"
                },
                "thumb_ms": {
                    "url": "https://img.esa.io/uploads/production/users/1/icon/thumb_ms_402685a258cf2a33c1d6c13a89adec92.png"
                },
                "thumb_m": {
                    "url": "https://img.esa.io/uploads/production/users/1/icon/thumb_m_402685a258cf2a33c1d6c13a89adec92.png"
                },
                "thumb_l": {
                    "url": "https://img.esa.io/uploads/production/users/1/icon/thumb_l_402685a258cf2a33c1d6c13a89adec92.png"
                }
            },
            "name": "Atsuo Fukaya",
            "screen_name": "fukayatsu"
        }
    });

    // Webhookペイロードのパース
    let result = parse_webhook_payload(payload);

    // 結果の検証
    assert!(result.is_ok(), "Webhookペイロードのパースに失敗しました");

    let webhook_payload = result.unwrap();

    // 各フィールドの検証
    assert_eq!(webhook_payload.kind, "post_create");
    assert_eq!(webhook_payload.team.name, "esa");
    assert_eq!(webhook_payload.post.name, "foo/bar/たいとる #tag1 #tag2");
    assert_eq!(webhook_payload.post.body_md, "ほんぶん");
    assert_eq!(webhook_payload.post.body_html, "<p>ほんぶん</p>\n");
    assert_eq!(webhook_payload.post.message.unwrap(), "Create post.");
    assert_eq!(webhook_payload.post.wip, false);
    assert_eq!(webhook_payload.post.number, 1253);
    assert_eq!(
        webhook_payload.post.url,
        "https://example.esa.io/posts/1253"
    );
    assert_eq!(webhook_payload.user.name, "Atsuo Fukaya");
    assert_eq!(webhook_payload.user.screen_name, "fukayatsu");
    assert_eq!(
        webhook_payload.user.icon.as_ref().unwrap().url,
        "https://img.esa.io/uploads/production/users/1/icon/402685a258cf2a33c1d6c13a89adec92.png"
    );
}

#[test]
fn test_parse_webhook_payload_invalid_kind() {
    // 無効なkindを持つペイロード
    let payload = json!({
        "kind": "invalid_kind",
        "team": {
            "name": "esa"
        },
        "post": {
            "name": "foo/bar/たいとる #tag1 #tag2",
            "body_md": "ほんぶん",
            "body_html": "<p>ほんぶん</p>\n",
            "message": "Create post.",
            "wip": false,
            "number": 1253,
            "url": "https://example.esa.io/posts/1253"
        },
        "user": {
            "name": "Atsuo Fukaya",
            "screen_name": "fukayatsu"
        }
    });

    // Webhookペイロードのパース
    let result = parse_webhook_payload(payload);

    // 結果の検証 - エラーが返されることを確認
    assert!(
        result.is_err(),
        "無効なkindを持つペイロードがエラーを返さなかった"
    );
}

#[test]
fn test_parse_webhook_payload_post_update() {
    // テストデータ
    let payload = json!({
        "kind": "post_update",
        "team": {
            "name": "esa"
        },
        "post": {
            "name": "foo/bar/たいとる #tag1 #tag2",
            "body_md": "ほんぶん",
            "body_html": "<p>ほんぶん</p>\n",
            "message": "Update post.",
            "wip": false,
            "number": 1253,
            "url": "https://example.esa.io/posts/1253",
            "diff_url": "https://example.esa.io/posts/1253/revisions/3"
        },
        "user": {
            "icon": {
                "url": "https://img.esa.io/uploads/production/users/1/icon/402685a258cf2a33c1d6c13a89adec92.png",
                "thumb_s": {
                    "url": "https://img.esa.io/uploads/production/users/1/icon/thumb_s_402685a258cf2a33c1d6c13a89adec92.png"
                },
                "thumb_ms": {
                    "url": "https://img.esa.io/uploads/production/users/1/icon/thumb_ms_402685a258cf2a33c1d6c13a89adec92.png"
                },
                "thumb_m": {
                    "url": "https://img.esa.io/uploads/production/users/1/icon/thumb_m_402685a258cf2a33c1d6c13a89adec92.png"
                },
                "thumb_l": {
                    "url": "https://img.esa.io/uploads/production/users/1/icon/thumb_l_402685a258cf2a33c1d6c13a89adec92.png"
                }
            },
            "name": "Atsuo Fukaya",
            "screen_name": "fukayatsu"
        }
    });

    // Webhookペイロードのパース
    let result = parse_webhook_payload(payload);

    // 結果の検証
    assert!(result.is_ok(), "Webhookペイロードのパースに失敗しました");

    let webhook_payload = result.unwrap();

    // 各フィールドの検証
    assert_eq!(webhook_payload.kind, "post_update");
    assert_eq!(webhook_payload.team.name, "esa");
    assert_eq!(webhook_payload.post.name, "foo/bar/たいとる #tag1 #tag2");
    assert_eq!(webhook_payload.post.body_md, "ほんぶん");
    assert_eq!(webhook_payload.post.body_html, "<p>ほんぶん</p>\n");
    assert_eq!(webhook_payload.post.message.unwrap(), "Update post.");
    assert_eq!(webhook_payload.post.wip, false);
    assert_eq!(webhook_payload.post.number, 1253);
    assert_eq!(
        webhook_payload.post.url,
        "https://example.esa.io/posts/1253"
    );
    assert_eq!(webhook_payload.user.name, "Atsuo Fukaya");
    assert_eq!(webhook_payload.user.screen_name, "fukayatsu");
    assert_eq!(
        webhook_payload.user.icon.as_ref().unwrap().url,
        "https://img.esa.io/uploads/production/users/1/icon/402685a258cf2a33c1d6c13a89adec92.png"
    );
}

#[test]
fn test_parse_webhook_payload_invalid_format() {
    // 無効な形式のペイロード
    let payload = json!("invalid");

    // Webhookペイロードのパース
    let result = parse_webhook_payload(payload);

    // 結果の検証 - エラーが返されることを確認
    assert!(
        result.is_err(),
        "無効な形式のペイロードがエラーを返さなかった"
    );
}
