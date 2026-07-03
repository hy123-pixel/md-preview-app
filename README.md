# MD Preview — Markdown 全功能预览编辑器

> 基于 Tauri v2 + Vue 3 + TypeScript 的跨平台 Markdown 编辑器，支持实时预览、自动更新、GitHub 发布、文件关联打开。

---

## 功能特性

### 编辑与预览
- ✅ **实时预览**：编辑时右侧即时渲染 HTML 预览
- ✅ **左右对比**：左源码右预览，支持拖动调整比例（`Cmd/Ctrl + 1`）
- ✅ **仅编辑模式**：隐藏预览，专注写作（`Cmd/Ctrl + 2`）
- ✅ **仅预览模式**：全屏预览最终效果（`Cmd/Ctrl + 3`）**— 默认启动模式**
- ✅ **同步滚动**：源码与预览滚动位置联动

### 文件操作
- ✅ **打开文件**：系统文件对话框选择 `.md` / `.markdown` 文件（`Cmd/Ctrl + O`）
- ✅ **打开文件夹**：打开包含 Markdown 的文件夹，显示文件树
- ✅ **新建文件**：创建空白 Markdown 文件（`Cmd/Ctrl + N`）
- ✅ **保存**：保存当前编辑内容（`Cmd/Ctrl + S`）
- ✅ **另存为**：将内容另存为新文件（`Cmd/Ctrl + Shift + S`）
- ✅ **拖拽打开**：支持从 Finder/Explorer 拖拽文件到窗口直接打开
- ✅ **右键打开**：注册 `.md` 文件默认打开程序，右键可用 MD Preview 打开
- ✅ **文件关联**：双击 `.md` 文件直接启动并打开内容

### 文件历史记录
- ✅ **自动记录**：每次打开文件自动记录到历史列表
- ✅ **持久化存储**：历史记录保存在 localStorage，重启后保留
- ✅ **显示路径**：历史菜单显示文件名 + 完整路径（路径过长时自动截断）
- ✅ **一键打开**：点击历史记录直接打开对应文件
- ✅ **单条删除**：hover 历史项显示删除按钮，点击移除该条记录
- ✅ **清除全部**：支持一键清空所有历史记录
- ✅ **最多保留 20 条**：自动淘汰最早记录

### Markdown 渲染支持
- ✅ **基础语法**：标题、粗体、斜体、删除线、代码、引用、列表、链接、图片、分隔线
- ✅ **表格**：完整表格渲染，支持对齐
- ✅ **代码块**：语法高亮，支持主流编程语言
- ✅ **任务列表**：`- [ ]` / `- [x]` 任务清单
- ✅ **数学公式**：行内 `$...$` 和块级 `$$...$$` 的 KaTeX 渲染
- ✅ **Mermaid 图表**：流程图、时序图、甘特图等
- ✅ **目录导航**：自动生成 TOC，点击跳转章节
- ✅ **Emoji**：GitHub 风格 Emoji `:shortcode:`

### 界面与交互
- ✅ **主题切换**：亮色 / 暗色 / 跟随系统
- ✅ **菜单栏**：文件、视图下拉菜单（含快捷键提示、历史记录）
- ✅ **工具栏隐藏**：支持 `Cmd/Ctrl + Shift + B` 隐藏/显示工具栏，按钮也可一键隐藏
- ✅ **文件树**：侧边栏显示文件夹结构（默认隐藏，按 `Cmd/Ctrl + B` 展开）
- ✅ **目录面板**：右侧显示文章大纲（默认隐藏，按 `Cmd/Ctrl + T` 展开）
- ✅ **状态栏**：显示文件名、词数、字符数、光标位置
- ✅ **暗色编辑器**：CodeMirror 6 暗色主题自动适配

### 自动更新
- ✅ **启动时自动检查**：App 启动 3 秒后静默检查更新
- ✅ **弹窗提示**：发现新版本时弹窗提示，支持一键下载安装并自动重启
- ✅ **手动检查**：支持菜单栏手动检查更新
- ✅ **GitHub 发布**：通过 GitHub Actions 自动构建 macOS + Windows 并发布

### 错误处理与权限
- ✅ **保存失败提示**：保存失败时弹窗提示具体原因
- ✅ **权限不足引导**：无法访问文件时，弹窗提示前往「系统设置 → 隐私与安全性 → 文件和文件夹」授予权限
- ✅ **右键打开失败提示**：文件关联打开失败时，给出明确的权限引导

### 跨平台
- ✅ **macOS**：原生 App Bundle + DMG 安装包（Apple Silicon + Intel）
- ✅ **Windows**：NSIS 安装程序 + MSI 安装包
- ✅ **文件关联**：macOS / Windows 均支持 `.md` 文件双击打开和右键打开
- ✅ **代码签名**：macOS ad-hoc 签名 + Tauri 更新签名验证

---

## 快捷键

| 功能 | macOS | Windows |
|------|-------|---------|
| 新建文件 | `Cmd + N` | `Ctrl + N` |
| 打开文件 | `Cmd + O` | `Ctrl + O` |
| 打开文件夹 | `Cmd + Shift + O` | `Ctrl + Shift + O` |
| 保存 | `Cmd + S` | `Ctrl + S` |
| 另存为 | `Cmd + Shift + S` | `Ctrl + Shift + S` |
| 左右对比 | `Cmd + 1` | `Ctrl + 1` |
| 仅编辑 | `Cmd + 2` | `Ctrl + 2` |
| 仅预览 | `Cmd + 3` | `Ctrl + 3` |
| 切换文件树 | `Cmd + B` | `Ctrl + B` |
| 切换目录 | `Cmd + T` | `Ctrl + T` |
| 切换工具栏 | `Cmd + Shift + B` | `Ctrl + Shift + B` |
| 撤销 | `Cmd + Z` | `Ctrl + Z` |
| 重做 | `Cmd + Shift + Z` | `Ctrl + Shift + Z` |
| 查找 | `Cmd + F` | `Ctrl + F` |

---

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri v2 (Rust) |
| 前端框架 | Vue 3 + TypeScript + Vite |
| 状态管理 | Pinia |
| 编辑器 | CodeMirror 6 |
| Markdown 解析 | marked v18 + marked-gfm-heading-id |
| 代码高亮 | highlight.js |
| 数学公式 | KaTeX |
| 图表渲染 | Mermaid |
| 分屏组件 | Splitpanes |
| 自动更新 | tauri-plugin-updater |
| 持久化 | localStorage（历史记录） |

---

## 项目结构

```
md-preview-app/
├── src/                          # Vue3 前端源码
│   ├── main.ts                   # 入口
│   ├── App.vue                   # 根组件
│   ├── stores/
│   │   └── editor.ts             # Pinia 状态管理（含历史记录持久化）
│   ├── components/
│   │   ├── Toolbar.vue           # 顶部工具栏 + 下拉菜单 + 历史记录
│   │   ├── EditorPane.vue        # CodeMirror 编辑器
│   │   ├── PreviewPane.vue       # Markdown 预览
│   │   ├── FileTree.vue          # 文件树
│   │   ├── TocPanel.vue          # 目录面板
│   │   ├── StatusBar.vue         # 底部状态栏
│   │   └── UpdateChecker.vue     # 自动更新检查器
│   ├── composables/
│   │   ├── useMarkdown.ts        # Markdown 渲染逻辑
│   │   ├── useFile.ts            # 文件操作（含错误弹窗）
│   │   └── useTheme.ts           # 主题管理
│   └── types/
│       └── mermaid.d.ts          # Mermaid 类型声明
├── src-tauri/                    # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json           # 含 updater 配置、签名、文件关联
│   ├── capabilities/
│   │   └── default.json          # 权限配置
│   └── src/
│       └── lib.rs                # 主入口（含文件关联/拖拽处理）
├── .github/workflows/
│   ├── build-macos.yml           # macOS CI 构建 + 签名 + 发布
│   └── build-windows.yml         # Windows CI 构建 + 签名 + 发布
├── scripts/
│   ├── release.py                # 一键发布脚本（版本 bump + Tag + 轮询 CI）
│   └── generate-latest-json.py   # 生成 latest.json 更新清单
├── tauri-signing.pub             # 更新签名公钥（minisign 格式）
├── tauri-signing.key             # 更新签名私钥（**不要提交到仓库**）
├── package.json
├── vite.config.ts
├── tsconfig.json
└── README.md
```

---

## 开发环境

### 前置依赖

- **Node.js** >= 18
- **Rust** >= 1.70

### 安装依赖

```bash
cd md-preview-app
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 构建

```bash
# macOS（当前架构）
npm run tauri build

# Windows（在 Windows 机器上）
npm run tauri build -- --target x86_64-pc-windows-msvc
```

构建产物：
- `src-tauri/target/release/bundle/macos/MD Preview.app`
- `src-tauri/target/release/bundle/dmg/MD Preview_1.0.0_aarch64.dmg`
- `src-tauri/target/release/bundle/nsis/MD Preview_1.0.0_x64-setup.exe`
- `src-tauri/target/release/bundle/msi/MD Preview_1.0.0_x64_en-US.msi`

---

## 自动更新配置

### 1. 签名密钥

密钥对已生成：
- `tauri-signing.pub` — 公钥（已嵌入 `tauri.conf.json`）
- `tauri-signing.key` — 私钥（**请添加到 GitHub Secrets**）

在 GitHub 仓库设置中添加以下 Secrets：
- `TAURI_SIGNING_PRIVATE_KEY` — 粘贴 `tauri-signing.key` 的内容（第二行 base64，不含 `untrusted comment` 行）
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` — 留空（空密码）
- `RELEASE_TOKEN` — 具有 `repo` 权限的 GitHub Personal Access Token

### 2. 发布流程

```bash
# 一键发布（自动 bump patch 版本）
python3 scripts/release.py patch

# 或指定版本
python3 scripts/release.py 1.0.1

# 或交互式输入版本
python3 scripts/release.py
```

发布流程：
1. 同步更新 `package.json` / `Cargo.toml` / `tauri.conf.json` 版本号
2. 提交并推送版本变更到 `main`
3. 创建并推送 Tag（触发 GitHub Actions）
4. 轮询 GitHub Actions 状态直到 macOS + Windows 构建完成
5. 自动上传构建产物到 GitHub Release
6. 自动生成 `latest.json` 更新清单

### 3. 更新检查

App 启动后自动检查更新。也可在菜单栏手动触发检查。

---

## 文件关联与右键打开

### macOS
- 安装 DMG 后，`.md` 文件会自动关联到 MD Preview
- 右键 `.md` 文件 → 「打开方式」→ MD Preview
- 拖拽 `.md` 文件到 MD Preview 窗口即可打开

### Windows
- 安装 EXE/MSI 后，`.md` 文件会自动关联到 MD Preview
- 右键 `.md` 文件 → 「打开方式」→ MD Preview
- 拖拽 `.md` 文件到 MD Preview 窗口即可打开

### 权限问题
如果在 macOS 上右键打开或拖拽打开时提示「无法读取文件」，请前往：
**系统设置 → 隐私与安全性 → 文件和文件夹 → MD Preview** → 开启所需目录的访问权限。

---

## 跨端数据说明

| 功能 | macOS | Windows |
|------|-------|---------|
| 原生菜单栏 | ✅ 自定义工具栏 + 下拉菜单 | ✅ 自定义工具栏 + 下拉菜单 |
| 文件关联 | ✅ `.md` / `.markdown` 双击/右键打开 | ✅ `.md` / `.markdown` 双击/右键打开 |
| 拖拽打开 | ✅ 支持 | ✅ 支持 |
| 暗色模式 | ✅ 跟随系统 + 手动切换 | ✅ 跟随系统 + 手动切换 |
| 自动更新 | ✅ 支持 | ✅ 支持 |
| 构建产物 | `.app` + `.dmg` | `.exe` + `.msi` |

---

## 许可

MIT License
