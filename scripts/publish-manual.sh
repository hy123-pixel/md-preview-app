#!/usr/bin/env bash
# MD Preview 手动发布脚本（上传到 GitHub Release）
#
# 用法：
#   export GITHUB_TOKEN="ghp_xxxxxxxxxxxx"
#   bash scripts/publish-manual.sh v1.0.0

set -euo pipefail

VERSION="${1:-v1.0.0}"
TAG="${VERSION}"
REPO="hy123-pixel/md-preview-app"
DMG_PATH="src-tauri/target/release/bundle/dmg/MD Preview_${VERSION#v}_aarch64.dmg"

if [ -z "${GITHUB_TOKEN:-}" ]; then
  echo "错误：请设置 GITHUB_TOKEN 环境变量"
  echo "  export GITHUB_TOKEN=\"ghp_xxxxxxxxxxxx\""
  echo "Token 从 https://github.com/settings/tokens 获取（需要 repo 权限）"
  exit 1
fi

if [ ! -f "$DMG_PATH" ]; then
  echo "错误：DMG 文件不存在：$DMG_PATH"
  echo "请先运行：npm run tauri build"
  exit 1
fi

echo "发布 $TAG 到 $REPO ..."

# 创建 Release
echo "1. 创建 GitHub Release ..."
RELEASE_JSON=$(curl -s -X POST \
  -H "Authorization: Bearer $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  "https://api.github.com/repos/$REPO/releases" \
  -d "{\"tag_name\":\"$TAG\",\"name\":\"MD Preview $TAG\",\"draft\":false,\"prerelease\":false}")

UPLOAD_URL=$(echo "$RELEASE_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['upload_url'].replace('{?name,label}',''))" 2>/dev/null || true)
RELEASE_ID=$(echo "$RELEASE_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['id'])" 2>/dev/null || true)

if [ -z "$UPLOAD_URL" ] || [ -z "$RELEASE_ID" ]; then
  echo "创建 Release 失败，可能已存在"
  # 尝试获取已有 Release
  RELEASE_JSON=$(curl -s \
    -H "Authorization: Bearer $GITHUB_TOKEN" \
    "https://api.github.com/repos/$REPO/releases/tags/$TAG")
  UPLOAD_URL=$(echo "$RELEASE_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['upload_url'].replace('{?name,label}',''))" 2>/dev/null || true)
  RELEASE_ID=$(echo "$RELEASE_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['id'])" 2>/dev/null || true)
fi

if [ -z "$UPLOAD_URL" ]; then
  echo "错误：无法获取 Release upload URL"
  exit 1
fi

# 上传 DMG
echo "2. 上传 DMG ..."
DMG_NAME=$(basename "$DMG_PATH")
curl -s -X POST \
  -H "Authorization: Bearer $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  -H "Content-Type: application/octet-stream" \
  --data-binary "@$DMG_PATH" \
  "${UPLOAD_URL}?name=${DMG_NAME// /%20}"

echo ""
echo "✅ 发布完成！"
echo "   Release: https://github.com/$REPO/releases/tag/$TAG"
