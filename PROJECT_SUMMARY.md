# esa-feedback プロジェクト概要

このドキュメントでは、esa-feedback プロジェクトの構造と実装の詳細について説明します。

## プロジェクトの目的

esa-feedback は、esa.io に投稿された記事に対して LLM（Gemini 等）を用いて自動的にフィードバックコメントを行うツールです。記事のパス（カテゴリ/タイトル）に基づいて異なるプロンプトを使用し、カスタマイズされたフィードバックを提供します。

## システム構成

```
esa.io --Webhook通知--> esa-feedback --API呼び出し--> Vertex AI
                                      |
                                      v
                        esa.io <--コメント投稿--
```

## プロジェクト構造

```
esa-feedback/
├── Cargo.toml                      # プロジェクト設定とライブラリ依存関係
├── src/
│   ├── main.rs                     # エントリーポイント
│   ├── config.rs                   # 設定管理
│   ├── error.rs                    # エラー定義
│   ├── server.rs                   # Webサーバー実装
│   ├── webhook.rs                  # Webhook処理
│   ├── handlers/                   # パターン別ハンドラー
│   │   ├── mod.rs                  # ハンドラーモジュール定義とルーティングロジック
│   │   ├── weekly_report/          # 週報ハンドラーモジュール
│   │   │   ├── mod.rs              # 週報処理ロジック
│   │   │   └── prompt.txt          # 週報用プロンプト
│   │   ├── development/            # 開発記事ハンドラーモジュール
│   │   │   ├── mod.rs              # 開発記事処理ロジック
│   │   │   └── prompt.txt          # 開発記事用プロンプト
│   │   └── default/                # デフォルトハンドラーモジュール
│   │       ├── mod.rs              # デフォルト処理ロジック
│   │       └── prompt.txt          # デフォルトプロンプト
│   ├── esa/                        # esa.io API連携
│   │   ├── mod.rs                  # モジュール定義
│   │   ├── client.rs               # APIクライアント
│   │   ├── models.rs               # データモデル
│   │   └── webhook.rs              # Webhookデータ処理
│   └── vertex_ai/                  # Vertex AI連携
│       ├── mod.rs                  # モジュール定義
│       ├── client.rs               # APIクライアント
│       └── models.rs               # データモデル
└── README.md                       # プロジェクト説明
```

## 主要コンポーネントの説明

### 1. Webhook 受信 (src/server.rs, src/webhook.rs)

- esa.io からの Webhook 通知を受信するエンドポイントを提供
- 通知データを解析し、記事情報を抽出
- 記事のパスに基づいて適切なハンドラーにルーティング

### 2. パターンマッチングとルーティング (src/handlers/mod.rs)

- 記事のパス（カテゴリ/タイトル）に基づいて適切なハンドラーを選択
- 現在実装されているパターン:
  - 週報パターン (`^週報/`)
  - 開発パターン (`^プロジェクト/開発/`)
  - デフォルトパターン (`.*`)

### 3. フィードバック生成 (src/handlers/\*/mod.rs)

- 各パターン用のハンドラーが、同じディレクトリにあるプロンプトファイルを読み込み
- Vertex AI API を使用して LLM によるフィードバックを生成
- 生成されたフィードバックを esa.io API を使用して元の記事にコメントとして投稿

### 4. esa.io API 連携 (src/esa/)

- esa.io API との通信を担当
- Webhook 通知の解析
- コメント投稿機能の提供

### 5. Vertex AI 連携 (src/vertex_ai/)

- Vertex AI API との通信を担当
- LLM によるフィードバック生成機能の提供

## 環境変数

以下の環境変数を設定する必要があります：

- `ESA_ACCESS_TOKEN`: esa.io の API アクセストークン（コメント投稿用）
- `ESA_TEAM_NAME`: esa.io のチーム名
- `VERTEX_AI_PROJECT_ID`: Google Cloud Project の ID
- `VERTEX_AI_LOCATION`: Vertex AI のロケーション（例: us-central1）
- `VERTEX_AI_MODEL`: 使用するモデル名（例: gemini-pro）

## 新しいパターンの追加方法

新しいパターンを追加するには：

1. `src/handlers/` ディレクトリに新しいパターン用のディレクトリを作成
2. `mod.rs` ファイルに処理ロジックを実装
3. `prompt.txt` ファイルにプロンプトを記述
4. `src/handlers/mod.rs` ファイルのパターンリストに新しいパターンを追加

## 実装上の注意点

1. **Axum の使用**: Web フレームワークとして axum 0.7 を使用しています。axum の最新バージョンでは、サーバーの起動方法やハンドラーの実装方法が変更されている可能性があります。

2. **非同期処理**: tokio を使用して非同期処理を実装しています。すべての I/O 操作（ファイル読み込み、API 呼び出しなど）は非同期で行われます。

3. **エラーハンドリング**: thiserror を使用してエラー型を定義しています。すべてのエラーは適切に処理され、ログに記録されます。

4. **ログ記録**: tracing を使用してログを記録しています。重要なイベントやエラーはすべてログに記録されます。

5. **設定管理**: 環境変数を使用して設定を管理しています。dotenv を使用して.env ファイルから環境変数を読み込むこともできます。

## デプロイ

このアプリケーションは Cloud Run にデプロイすることを想定しています。デプロイ手順は以下の通りです：

1. Dockerfile を作成
2. コンテナイメージをビルド
3. Google Cloud Run にデプロイ
4. 必要な環境変数を設定
5. esa.io の Webhook 設定でデプロイした URL を指定
