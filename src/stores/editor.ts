import { defineStore } from "pinia";
import { ref, computed } from "vue";

export interface FileInfo {
  path: string | null;
  name: string;
  content: string;
  isDirty: boolean;
}

export interface EditorSettings {
  theme: "light" | "dark" | "system";
  previewTheme: "github" | "vuepress" | "minimal";
  fontSize: number;
  tabSize: number;
  wordWrap: boolean;
  syncScroll: boolean;
  showToc: boolean;
  showLineNumbers: boolean;
}

export interface FileNode {
  name: string;
  path: string;
  isDirectory: boolean;
  children?: FileNode[];
}

export const useEditorStore = defineStore("editor", () => {
  // 文件信息
  const currentFile = ref<FileInfo>({
    path: null,
    name: "未命名.md",
    content: getWelcomeContent(),
    isDirty: false,
  });

  // 视图模式: split | editor-only | preview-only
  const viewMode = ref<"split" | "editor-only" | "preview-only">("preview-only");

  // 编辑器配置
  const settings = ref<EditorSettings>({
    theme: "system",
    previewTheme: "github",
    fontSize: 14,
    tabSize: 2,
    wordWrap: true,
    syncScroll: true,
    showToc: true,
    showLineNumbers: true,
  });

  // 文件树
  const openFolder = ref<string | null>(null);
  const selectedFile = ref<string | null>(null);
  const fileTree = ref<FileNode[]>([]);

  // 最近文件（从 localStorage 加载）
  const recentFiles = ref<string[]>([]);
  try {
    const stored = localStorage.getItem("md-preview-recent-files");
    if (stored) {
      recentFiles.value = JSON.parse(stored);
    }
  } catch {
    // 忽略解析错误
  }

  // 编辑器内容（用于与 CodeMirror 双向绑定）
  const editorContent = ref<string>(currentFile.value.content);

  // 计算属性
  const hasFileOpen = computed(() => currentFile.value.path !== null);
  const isUntitled = computed(() => currentFile.value.path === null);
  const displayTitle = computed(() => {
    const dirtyMark = currentFile.value.isDirty ? " ●" : "";
    return currentFile.value.name + dirtyMark;
  });

  // 方法
  function setContent(content: string) {
    editorContent.value = content;
    currentFile.value.content = content;
    currentFile.value.isDirty = true;
  }

  function loadFile(path: string | null, name: string, content: string) {
    currentFile.value = {
      path,
      name,
      content,
      isDirty: false,
    };
    editorContent.value = content;

    if (path) {
      addRecentFile(path);
    }
  }

  function markSaved(path: string | null, name: string) {
    currentFile.value.path = path;
    currentFile.value.name = name;
    currentFile.value.isDirty = false;
  }

  function newFile() {
    currentFile.value = {
      path: null,
      name: "未命名.md",
      content: "",
      isDirty: false,
    };
    editorContent.value = "";
  }

  function addRecentFile(path: string) {
    recentFiles.value = recentFiles.value.filter((f) => f !== path);
    recentFiles.value.unshift(path);
    if (recentFiles.value.length > 20) {
      recentFiles.value = recentFiles.value.slice(0, 20);
    }
    try {
      localStorage.setItem("md-preview-recent-files", JSON.stringify(recentFiles.value));
    } catch {
      // 忽略写入错误
    }
  }

  function removeRecentFile(path: string) {
    recentFiles.value = recentFiles.value.filter((f) => f !== path);
    try {
      localStorage.setItem("md-preview-recent-files", JSON.stringify(recentFiles.value));
    } catch {
      // 忽略写入错误
    }
  }

  function clearRecentFiles() {
    recentFiles.value = [];
    try {
      localStorage.removeItem("md-preview-recent-files");
    } catch {
      // 忽略删除错误
    }
  }

  function setViewMode(mode: "split" | "editor-only" | "preview-only") {
    viewMode.value = mode;
  }

  function updateSettings(partial: Partial<EditorSettings>) {
    settings.value = { ...settings.value, ...partial };
  }

  function setFileTree(tree: FileNode[]) {
    fileTree.value = tree;
  }

  return {
    currentFile,
    viewMode,
    settings,
    openFolder,
    selectedFile,
    fileTree,
    recentFiles,
    editorContent,
    hasFileOpen,
    isUntitled,
    displayTitle,
    setContent,
    loadFile,
    markSaved,
    newFile,
    addRecentFile,
    removeRecentFile,
    clearRecentFiles,
    setViewMode,
    updateSettings,
    setFileTree,
  };
});

function getWelcomeContent(): string {
  return `# 欢迎使用 MD Preview

> 一款轻量、高效的 Markdown 预览与编辑器

## 基础功能

- **打开文件**：\`Cmd + O\` (macOS) / \`Ctrl + O\` (Windows)
- **新建文件**：\`Cmd + N\` / \`Ctrl + N\`
- **保存文件**：\`Cmd + S\` / \`Ctrl + S\`
- **视图切换**：工具栏可切换「左右对比 / 仅编辑 / 仅预览」模式

## Markdown 支持

### 代码块
\`\`\`typescript
const hello = "world";
console.log(hello);
\`\`\`

### 表格

| 功能 | 状态 |
|------|------|
| 实时预览 | ✅ |
| 左右对比 | ✅ |
| 编辑保存 | ✅ |
| 目录导航 | ✅ |
| 数学公式 | ✅ |
| Mermaid 图表 | ✅ |

### 任务列表

- [x] 核心编辑器
- [x] 实时预览
- [x] 文件操作
- [x] 目录导航
- [ ] 插件扩展

### 数学公式

行内公式：$E = mc^2$

块级公式：

$$
\\int_{-\\infty}^{+\\infty} e^{-x^2} dx = \\sqrt{\\pi}
$$

### Mermaid 流程图

\`\`\`mermaid
graph TD
    A[开始] --> B{判断}
    B -->|条件1| C[处理1]
    B -->|条件2| D[处理2]
    C --> E[结束]
    D --> E
\`\`\`

---

**提示**：你可以直接拖拽 Markdown 文件到窗口打开，或通过菜单栏进行文件操作。
`;
}
