# Markdown Preview Editor App — 基础需求文档

## 项目概述

**名称**: MD Preview  
**技术栈**: Tauri v2 + Vue 3 + TypeScript + Vite  
**目标平台**: macOS (arm64/x86_64), Windows (x86_64)  
**核心定位**: 一款轻量、高效的 Markdown 全功能预览与编辑器

---

## 功能需求

### 1. 文件操作

| 功能 | 描述 | 快捷键 |
|------|------|--------|
| 打开文件 | 通过系统文件对话框选择 .md/.markdown 文件 | Cmd/Ctrl + O |
| 打开文件夹 | 打开包含 Markdown 的文件夹，显示文件树 | Cmd/Ctrl + Shift + O |
| 新建文件 | 创建空白的 Markdown 文件 | Cmd/Ctrl + N |
| 保存 | 保存当前编辑内容 | Cmd/Ctrl + S |
| 另存为 | 将当前内容另存为新文件 | Cmd/Ctrl + Shift + S |
| 最近文件 | 菜单栏显示最近打开的文件列表 | - |
| 拖拽打开 | 支持从 Finder/Explorer 拖拽文件到窗口 | - |
| 双击打开 | 注册 .md 文件关联，双击默认用本 App 打开 | - |

### 2. 编辑功能

| 功能 | 描述 |
|------|------|
| 源码编辑 | 完整的 Markdown 源码编辑器，支持语法高亮 |
| 实时预览 | 编辑时右侧实时渲染 HTML 预览 |
| 左右对比 | 左右分屏，左侧源码右侧预览（默认模式） |
| 仅预览模式 | 隐藏编辑器，全屏预览最终效果 |
| 仅编辑模式 | 隐藏预览，专注编辑 |
| 同步滚动 | 左右分屏时，源码和预览滚动位置同步 |
| 查找替换 | 支持查找/替换文本 | Cmd/Ctrl + F |
| 撤销重做 | 支持多步撤销/重做 | Cmd/Ctrl + Z / Cmd/Ctrl + Shift + Z |

### 3. Markdown 渲染支持

| 类型 | 支持情况 |
|------|----------|
| 基础语法 | 标题、段落、粗体、斜体、删除线、代码、引用、列表、链接、图片、分隔线 |
| 表格 | 完整表格渲染，支持对齐 |
| 代码块 | 语法高亮，支持所有主流语言 |
| 任务列表 | - [ ] / - [x] 任务清单 |
| 数学公式 | 行内 $...$ 和块级 $$...$$ 的 KaTeX 渲染 |
| 目录 | 自动生成目录（TOC） |
| Mermaid | 流程图、时序图、甘特图等 Mermaid 图表 |
| 脚注 | 支持 Markdown 脚注 |
| Emoji | 支持 GitHub 风格 Emoji :shortcode: |
| 目录锚点 | 点击目录跳转到对应章节 |

### 4. 界面与交互

| 功能 | 描述 |
|------|------|
| 主题切换 | 支持亮色/暗色/跟随系统三种模式 |
| 预览主题 | 多种预览 CSS 主题可选（GitHub、VuePress、文档等） |
| 字体大小 | 编辑器字体大小可调 |
| 分屏比例 | 可拖动调整左右分屏比例 |
| 全屏模式 | 支持 macOS 全屏 |
| 窗口状态记忆 | 重启后恢复窗口大小、位置、分屏比例 |

### 5. 菜单栏（macOS）/ 菜单（Windows）

```
App 菜单 (macOS) / 文件菜单 (Windows)
  ├─ 关于 MD Preview
  ├─ 设置...
  └─ 退出

文件
  ├─ 新建 (Cmd+N)
  ├─ 打开 (Cmd+O)
  ├─ 打开文件夹 (Cmd+Shift+O)
  ├─ 打开最近文件
  ├─ 保存 (Cmd+S)
  ├─ 另存为 (Cmd+Shift+S)
  └─ 关闭文件

编辑
  ├─ 撤销 (Cmd+Z)
  ├─ 重做 (Cmd+Shift+Z)
  ├─ 剪切 (Cmd+X)
  ├─ 复制 (Cmd+C)
  ├─ 粘贴 (Cmd+V)
  ├─ 全选 (Cmd+A)
  └─ 查找 (Cmd+F)

视图
  ├─ 左右对比模式
  ├─ 仅编辑模式
  ├─ 仅预览模式
  ├─ 同步滚动 (开/关)
  ├─ 显示目录 (开/关)
  ├─ 亮色模式
  ├─ 暗色模式
  └─ 跟随系统

帮助
  └─ Markdown 语法参考
```

---

## 技术架构

### 前端
- **Vue 3** + **TypeScript** + **Vite**
- **CodeMirror 6** — 源码编辑器（语法高亮、自动补全）
- **marked** + **marked-gfm-heading-id** — Markdown 解析
- **highlight.js** — 代码块语法高亮
- **KaTeX** — 数学公式渲染
- **mermaid** — 图表渲染
- **Pinia** — 状态管理
- **Splitpanes** — 分屏面板

### 后端（Tauri Rust）
- **Tauri v2** — 跨平台桌面框架
- **tauri-plugin-dialog** — 系统文件对话框
- **tauri-plugin-fs** — 文件系统读写
- **tauri-plugin-store** — 本地配置存储
- **tauri-plugin-os** — 系统信息

---

## 项目结构

```
md-preview-app/
├── src/                          # Vue3 前端源码
│   ├── main.ts
│   ├── App.vue
│   ├── stores/
│   │   └── editor.ts             # Pinia 状态管理
│   ├── components/
│   │   ├── EditorPane.vue        # 编辑器面板
│   │   ├── PreviewPane.vue       # 预览面板
│   │   ├── FileTree.vue          # 文件树
│   │   ├── TocPanel.vue          # 目录面板
│   │   ├── Toolbar.vue           # 顶部工具栏
│   │   └── StatusBar.vue         # 底部状态栏
│   ├── composables/
│   │   ├── useMarkdown.ts        # Markdown 渲染逻辑
│   │   ├── useFile.ts            # 文件操作
│   │   └── useTheme.ts           # 主题管理
│   ├── styles/
│   │   ├── themes/
│   │   │   ├── github-light.css
│   │   │   └── github-dark.css
│   │   └── editor.css
│   └── utils/
│       └── markdown.ts           # Markdown 工具函数
├── src-tauri/                    # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json
│   └── src/
│       └── main.rs
├── index.html
├── vite.config.ts
├── tsconfig.json
├── package.json
└── README.md
```

---

## 数据结构

### 编辑器状态 (Pinia Store)

```typescript
interface EditorState {
  // 文件信息
  currentFile: {
    path: string | null;
    name: string;
    content: string;
    isDirty: boolean;
  };
  
  // 视图模式
  viewMode: 'split' | 'editor-only' | 'preview-only';
  
  // 编辑器配置
  settings: {
    theme: 'light' | 'dark' | 'system';
    previewTheme: 'github' | 'vuepress' | 'minimal';
    fontSize: number;        // 默认 14
    tabSize: number;         // 默认 2
    wordWrap: boolean;       // 默认 true
    syncScroll: boolean;     // 默认 true
    showToc: boolean;        // 默认 true
    showLineNumbers: boolean; // 默认 true
  };
  
  // 文件树
  openFolder: string | null;
  selectedFile: string | null;
  fileTree: FileNode[];
  
  // 最近文件
  recentFiles: string[];
}
```

---

## 跨平台适配

### macOS
- 原生菜单栏 (Menu Bar)
- 文件关联 (.md → MD Preview)
- 暗色模式跟随系统
- 沙盒安全策略

### Windows
- 系统菜单
- 文件关联注册
- 暗色模式跟随系统
- 窗口拖拽支持

---

## 构建命令

```bash
# 开发
npm run tauri dev

# 构建 macOS
npm run tauri build -- --target universal-apple-darwin

# 构建 Windows
npm run tauri build -- --target x86_64-pc-windows-msvc
```
