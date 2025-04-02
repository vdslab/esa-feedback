# esa-feedback

esa-feedback は、esa.io に投稿された記事に対して LLM（Gemini 等）を用いて自動的にフィードバックコメントを行うツールです。

## 概要

このツールは、esa.io の webhook を利用して記事の投稿・更新通知を受け取り、記事の内容を Vertex AI の API に送信して LLM によるフィードバックを生成し、そのフィードバックを esa.io の API を使用して元の記事にコメントとして投稿します。

記事のパス（カテゴリ/タイトル）に基づいて異なるプロンプトを使用することができ、各パターンに対して専用の処理を実装することも可能です。

## 機能

- esa.io の webhook による記事投稿・更新通知の受信
- 記事のパス（カテゴリ/タイトル）に基づくパターンマッチングとルーティング
- Vertex AI API を使用した LLM によるフィードバック生成
- esa.io API を使用したコメント投稿

## 環境変数

以下の環境変数を設定する必要があります：

- `ESA_ACCESS_TOKEN`: esa.io の API アクセストークン（コメント投稿用）
- `ESA_TEAM_NAME`: esa.io のチーム名
- `VERTEX_AI_PROJECT_ID`: Google Cloud Project の ID
- `VERTEX_AI_LOCATION`: Vertex AI のロケーション（例: us-central1）
- `VERTEX_AI_MODEL`: 使用するモデル名（例: gemini-pro）

## セットアップ

### 前提条件

- Rust と Cargo がインストールされていること
- esa.io のアクセストークン
- Google Cloud Platform のアカウントと Vertex AI API へのアクセス権

### インストール

```bash
# リポジトリをクローン
git clone https://github.com/yourusername/esa-feedback.git
cd esa-feedback

# 環境変数を設定
cp .env.example .env
# .env ファイルを編集して必要な情報を入力

# ビルド
cargo build --release
```

## 実行

```bash
# 開発環境での実行
cargo run

# 本番環境での実行
./target/release/esa-feedback
```

## パターンとプロンプト

現在、以下のパターンとプロンプトが実装されています：

1. 週報パターン（`^週報/`）

   - 週報専用のフィードバックプロンプトを使用

2. 開発パターン（`^プロジェクト/開発/`）

   - 開発記事専用のフィードバックプロンプトを使用

3. デフォルトパターン（`.*`）
   - 一般的なフィードバックプロンプトを使用

## 新しいパターンの追加方法

新しいパターンを追加するには：

1. `src/handlers/` ディレクトリに新しいパターン用のディレクトリを作成
2. `mod.rs` ファイルに処理ロジックを実装
3. `prompt.txt` ファイルにプロンプトを記述
4. `src/handlers/mod.rs` ファイルのパターンリストに新しいパターンを追加

詳細は [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) を参照してください。

## CI/CD

### GitHub Actions

このプロジェクトには GitHub Actions を使用した継続的インテグレーション（CI）の設定が含まれています。この設定により、コードの変更がプッシュされたり、プルリクエストが作成されたりするたびに自動的にテストが実行されます。

GitHub Actions の設定は `.github/workflows/rust-tests.yml` ファイルで定義されています。

#### 実行されるチェック

- コードフォーマットのチェック（`cargo fmt --check`）
- Clippy による静的解析（`cargo clippy -- -D warnings`）
- 単体テストと統合テスト（`cargo test`）

#### 必要な GitHub シークレットの設定

テストを正常に実行するには、以下の GitHub シークレットを設定する必要があります：

1. GitHub リポジトリの Settings > Secrets and variables > Actions に移動
2. 以下のシークレットを追加：
   - `ESA_ACCESS_TOKEN`: esa.io の API アクセストークン
   - `ESA_TEAM_NAME`: esa.io のチーム名
   - `VERTEX_AI_PROJECT_ID`: Google Cloud Project の ID
   - `VERTEX_AI_LOCATION`: Vertex AI のロケーション（デフォルト: us-central1）
   - `VERTEX_AI_MODEL`: 使用するモデル名（デフォルト: gemini-pro）

## デプロイ

このアプリケーションは Cloud Run にデプロイすることを想定しています。

```bash
# Docker イメージのビルド
docker build -t gcr.io/your-project-id/esa-feedback .

# Google Container Registry へのプッシュ
docker push gcr.io/your-project-id/esa-feedback

# Cloud Run へのデプロイ
gcloud run deploy esa-feedback \
  --image gcr.io/your-project-id/esa-feedback \
  --platform managed \
  --region us-central1 \
  --set-env-vars ESA_ACCESS_TOKEN=your_token,ESA_TEAM_NAME=your_team,VERTEX_AI_PROJECT_ID=your_project_id,VERTEX_AI_LOCATION=us-central1,VERTEX_AI_MODEL=gemini-pro
```

## esa.io の Webhook 設定

1. esa.io の管理画面から Webhook 設定ページにアクセス
2. 新しい Webhook を追加
3. URL に Cloud Run のデプロイ URL + `/webhook` を指定
4. イベントタイプとして「記事作成時」と「記事更新時」を選択
5. 保存

## ライセンス

[LICENSE](LICENSE) ファイルを参照してください。

## 詳細情報

プロジェクトの詳細な構造と実装については [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) を参照してください。
