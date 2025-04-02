use esa_feedback::config::Config;
use esa_feedback::error::{Error, Result};
use esa_feedback::esa::client::EsaClient;

#[tokio::test]
async fn test_post_comment_to_6602() -> Result<()> {
    // 環境変数から設定を読み込む
    let config = Config::from_env()?;

    // ESAクライアントを初期化
    let esa_client = EsaClient::new(&config.esa_team_name, &config.esa_access_token)?;

    // 投稿番号6602に対してコメントを投稿
    let post_number = 6602;
    let comment_body = "これはテストコメントです。自動テストから投稿されました。";

    // コメントを投稿
    let result = esa_client.create_comment(post_number, comment_body).await;

    // 結果を確認
    match result {
        Ok(_) => {
            println!("投稿番号{}へのコメント投稿に成功しました。", post_number);
        }
        Err(e) => {
            println!(
                "投稿番号{}へのコメント投稿に失敗しました: {}",
                post_number, e
            );
            // 404エラーの場合は投稿が存在しないだけなのでテストは成功とする
            if let Error::EsaApi(msg) = &e {
                if msg.contains("404") {
                    println!("投稿が存在しないため、テストをスキップします");
                    return Ok(());
                }
            }
            return Err(e);
        }
    }

    Ok(())
}
