# MD Preview 发布指南

## 方式一：手动发布（推荐第一次使用）

### 1. 创建 GitHub 仓库

访问 https://github.com/new 创建仓库：
- Repository name: `md-preview-app`
- Visibility: **Public**
- 不勾选 README / .gitignore / license

### 2. 推送代码到 GitHub

```bash
cd /Volumes/UGREEN\ 1TB\ Media/Sync/Product/X/md-preview-app
git remote add origin git@github.com:hy123-pixel/md-preview-app.git
git branch -M main
git push -u origin main
```

### 3. 设置 GitHub Secrets

进入仓库 → Settings → Secrets and variables → Actions → New repository secret：

| Secret 名称 | 值 |
|------------|-----|
| `TAURI_SIGNING_PRIVATE_KEY` | `tauri-signing.key` 文件的全部内容 |
| `RELEASE_TOKEN` | GitHub Personal Access Token（见下方创建步骤） |

创建 GitHub Personal Access Token：
1. 访问 https://github.com/settings/tokens/new
2. 勾选 `repo`（Full control of private repositories）
3. 生成 Token，复制到 `RELEASE_TOKEN` Secret 中

### 4. 创建 GitHub Release 并上传 DMG（手动方式）

如果你没有设置 GitHub Actions，可以手动上传：

```bash
# 设置你的 GitHub Token（从 https://github.com/settings/tokens 获取）
export GITHUB_TOKEN="ghp_xxxxxxxxxxxxxxxxxxxx"

# 运行手动发布脚本
bash scripts/publish-manual.sh v1.0.0
```

### 5. 使用 GitHub Actions 自动发布（推荐后续版本）

```bash
# 设置 Token 环境变量（或写入 .env 文件）
export RELEASE_TOKEN="ghp_xxxxxxxxxxxxxxxxxxxx"

# 运行一键发布脚本
python3 scripts/release.py patch
```

这会：
1. 自动 bump 版本号
2. 提交并推送代码
3. 创建 Tag 触发 GitHub Actions
4. 自动构建 macOS 和 Windows 版本
5. 上传到 Release 并生成 `latest.json`

---

## 方式二：仅手动上传 DMG（最快）

如果你不想设置 CI，只想快速分享 App：

1. 访问 https://github.com/hy123-pixel/md-preview-app/releases/new
2. 输入 Tag version: `v1.0.0`
3. 输入 Release title: `MD Preview v1.0.0`
4. 上传文件 `src-tauri/target/release/bundle/dmg/MD Preview_1.0.0_aarch64.dmg`
5. 点击 Publish release

---

## 签名密钥说明

- **公钥** `tauri-signing.pub`：已嵌入 `src-tauri/tauri.conf.json`，已提交到 git
- **私钥** `tauri-signing.key`：已添加到 `.gitignore`，**不要提交到 git**
- 私钥需要作为 GitHub Secret `TAURI_SIGNING_PRIVATE_KEY` 设置，用于 CI 签名

---

## 快速验证（本地测试）

```bash
# 检查日志
/Applications/MD\ Preview.app/Contents/MacOS/md-preview-app

# 右键打开文件测试
cat /Applications/MD\ Preview.app/Contents/MacOS/logs/app.log
```
