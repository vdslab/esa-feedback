#!/bin/bash

# Configuration
PORT=8080
KIND="post_create"
WIP=false

# Get current date in YYYY/MM/DD format
CURRENT_DATE=$(date +"%Y/%m/%d")
POST_NAME="週報/$CURRENT_DATE/週報"

# Default content for weekly report
POST_CONTENT=$(cat <<EOF
# 目標トロフィー

- [ ] 緑色コーダーになった（達成日：〇〇〇〇年〇〇月〇〇日）
- [ ] 技術記事を1本公開した（達成日：〇〇〇〇年〇〇月〇〇日）

# 今週達成したトロフィー

なし

# 今週やったこと

- [x] チームMTG
- [x] バイト
- [ ] 課題提出

# 今週学んだこと・わかったこと

- Rustの非同期処理について学んだ

# 来週やること

- [ ] チーム発表の準備
- [ ] プロジェクトの実装を進める

# 今週見た面白いビジュアライゼーション

- 

# 所感

- テスト投稿です
EOF
)

# Escape JSON special characters in content
POST_CONTENT_ESCAPED=$(echo "$POST_CONTENT" | sed 's/\\/\\\\/g' | sed 's/"/\\"/g' | sed ':a;N;$!ba;s/\n/\\n/g')

# Create JSON payload
PAYLOAD=$(cat <<EOF
{
  "kind": "$KIND",
  "team": {
    "name": "vdslab"
  },
  "post": {
    "name": "$POST_NAME",
    "body_md": "$POST_CONTENT_ESCAPED",
    "body_html": "<p>HTML content</p>",
    "message": "Create post.",
    "wip": $WIP,
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
}
EOF
)

# Send the webhook request
echo "Sending $KIND webhook to http://localhost:$PORT/webhook"
echo "Post name: $POST_NAME"
echo "WIP: $WIP"

curl -X POST "http://localhost:$PORT/webhook" \
  -H "Content-Type: application/json" \
  -H "User-Agent: esa-Hookshot/v1" \
  -H "X-Esa-Delivery: $(date +%s)" \
  -d "$PAYLOAD" \
  -v
