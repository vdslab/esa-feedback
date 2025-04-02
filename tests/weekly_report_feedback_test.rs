use esa_feedback::config::Config;
use esa_feedback::error::Result;
use esa_feedback::vertex_ai::client::VertexAiClient;

#[tokio::test]
async fn test_generate_feedback() -> Result<()> {
    // 環境変数から設定を読み込む
    let config = Config::from_env()?;

    // 週報の内容
    let weekly_report_content = include_str!("test_data/weekly_report_content.txt");

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

    // フィードバックの内容を確認
    println!("フィードバックの内容:");
    println!("{}", feedback);

    Ok(())
}
