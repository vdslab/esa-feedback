use esa_feedback::config::Config;
use esa_feedback::error::Result;
use esa_feedback::vertex_ai::client::VertexAiClient;

#[tokio::test]
async fn test_generate_feedback() -> Result<()> {
    // 環境変数から設定を読み込む
    let config = Config::from_env()?;

    // 週報の内容
    let weekly_report_content = r#"# 目標トロフィー

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

- 疲労困憊
"#;

    // プロンプトを読み込む
    let prompt = include_str!("../src/handlers/weekly_report/prompt.txt");

    // Vertex AIクライアントを初期化
    let vertex_client = VertexAiClient::new(
        &config.vertex_ai_project_id,
        &config.vertex_ai_location,
        &config.vertex_ai_model,
    )?;

    // フィードバックを生成
    println!("Vertex AIからフィードバックを生成中...");
    let feedback = vertex_client
        .generate_feedback(weekly_report_content, prompt)
        .await?;
    println!("フィードバック生成完了");

    // フィードバックが生成されたことを確認
    assert!(!feedback.is_empty(), "フィードバックが空です");

    // フィードバックに期待される文字列が含まれていることを確認
    assert!(
        feedback.contains("週報フィードバック"),
        "フィードバックに「週報フィードバック」が含まれていません"
    );

    // 各セクションのフィードバックが含まれていることを確認
    let expected_sections = [
        "全体:",
        "1. 目標トロフィー:",
        "2. 今週達成したトロフィー:",
        "3. 今週やったこと",
        "4. 今週学んだこと",
        "5. 来週やること",
        "6. 今週見た面白いビジュアライゼーション:",
        "7. 所感",
    ];

    for section in expected_sections.iter() {
        assert!(
            feedback.contains(section),
            "フィードバックに「{}」セクションが含まれていません",
            section
        );
    }

    println!("フィードバックの内容:");
    println!("{}", feedback);

    Ok(())
}
