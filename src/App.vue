<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { Splitpanes, Pane } from "splitpanes";
import { useEditorStore } from "./stores/editor";
import { useFile } from "./composables/useFile";
import { useMarkdown } from "./composables/useMarkdown";
import { useTheme } from "./composables/useTheme";
import { ask, message } from "@tauri-apps/plugin-dialog";
import Toolbar from "./components/Toolbar.vue";
import EditorPane from "./components/EditorPane.vue";
import PreviewPane from "./components/PreviewPane.vue";
import StatusBar from "./components/StatusBar.vue";
import FileTree from "./components/FileTree.vue";
import TocPanel from "./components/TocPanel.vue";
import UpdateChecker from "./components/UpdateChecker.vue";

const store = useEditorStore();
const file = useFile();
const markdown = useMarkdown();
const theme = useTheme();

const isMac = /macintosh|mac os x/i.test(navigator.userAgent);
const showSidebar = ref(false);
const showTocPanel = ref(false);
const showToolbar = ref(!isMac); // macOS 上隐藏内部工具栏，使用系统菜单

// 禅模式：记录隐藏前的面板状态，用于恢复
const zenModeBefore = ref({
  showToolbar: !isMac,
  showSidebar: false,
  showTocPanel: false,
});

// 监听内容变化，重新渲染 Markdown（300ms 防抖）
let debounceTimer: ReturnType<typeof setTimeout> | null = null;
watch(() => store.editorContent, async (newVal) => {
  if (debounceTimer) clearTimeout(debounceTimer);
  debounceTimer = setTimeout(async () => {
    await markdown.render(newVal);
  }, 300);
}, { immediate: true });

// 初始化
onMounted(() => {
  theme.initSystemListener();
  markdown.render(store.editorContent);
  setupKeyboardShortcuts();
  setupTauriEvents();
  updateWindowTitle();
});

// 更新窗口标题
watch(() => store.displayTitle, updateWindowTitle);

async function updateWindowTitle() {
  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const win = getCurrentWindow();
    await win.setTitle(store.displayTitle || "MD Preview");
  } catch {
    // 非 Tauri 环境忽略
  }
}

// 监听 Tauri 事件（文件关联打开、拖拽打开、系统菜单、错误）
async function setupTauriEvents() {
  try {
    const { listen } = await import("@tauri-apps/api/event");

    // 监听文件打开事件（来自 Rust 后端：文件关联打开）
    // 注意：不自动切换视图，保持用户当前选择的视图模式
    listen("file-open", (event: any) => {
      const payload = event.payload as { path: string; name: string; content: string };
      if (payload && payload.path) {
        store.loadFile(payload.path, payload.name, payload.content);
      }
    });

    // 监听文件打开错误
    listen("file-open-error", async (event: any) => {
      const payload = event.payload as { path: string; error: string };
      if (payload) {
        await ask(
          `${payload.error}\n\n路径: ${payload.path}\n\n请前往「系统设置 → 隐私与安全性 → 文件和文件夹」授予 MD Preview 文件访问权限。`,
          { title: "无法打开文件", kind: "warning" }
        );
      }
    });

    // 监听拖拽文件事件（tauri://drag-drop）
    listen("tauri://drag-drop", (event: any) => {
      const payload = event.payload as { paths: string[] };
      if (payload && payload.paths && payload.paths.length > 0) {
        const path = payload.paths[0];
        const pathLower = path.toLowerCase();
        if (pathLower.endsWith(".md") || pathLower.endsWith(".markdown") || pathLower.endsWith(".txt")) {
          handleOpenFileByPath(path);
        }
      }
    });

    // 监听系统菜单事件（macOS 原生菜单）
    listen("menu-action", (event: any) => {
      const action = event.payload as string;
      handleMenuAction(action);
    });

    // 通知 Rust 前端已就绪，可以 emit 缓存的文件打开事件
    const { emit } = await import("@tauri-apps/api/event");
    emit("app-ready", {});
  } catch {
    // 非 Tauri 环境忽略
  }
}

// 全局键盘快捷键
function setupKeyboardShortcuts() {
  const isMac = navigator.platform.toLowerCase().includes("mac");
  const modKey = isMac ? "metaKey" : "ctrlKey";

  document.addEventListener("keydown", async (e) => {
    // Cmd/Ctrl + Shift + B: 切换工具栏显示/隐藏（Windows 专用）
    if (e.key === "b" && e[modKey] && e.shiftKey && !isMac) {
      e.preventDefault();
      showToolbar.value = !showToolbar.value;
      return;
    }

    // Cmd/Ctrl + Shift + H: 禅模式（一键隐藏/显示所有面板）
    if (e.key === "h" && e[modKey] && e.shiftKey) {
      e.preventDefault();
      handleZenMode();
      return;
    }

    // Cmd/Ctrl + / (或 Cmd/Ctrl + ?): 显示快捷键帮助
    if ((e.key === "?" || e.key === "/") && e[modKey]) {
      e.preventDefault();
      handleShowHelp();
      return;
    }

    // 忽略在编辑器内部的快捷键（由 CodeMirror 处理）
    const target = e.target as HTMLElement;
    if (target && target.closest(".cm-editor")) {
      // 允许 Cmd/Ctrl+S 通过（保存）
      if (e.key === "s" && e[modKey] && !e.shiftKey) {
        e.preventDefault();
        await handleSaveFile();
        return;
      }
      // 允许 Cmd/Ctrl+O 通过（打开文件）
      if (e.key === "o" && e[modKey] && !e.shiftKey) {
        e.preventDefault();
        await handleOpenFile();
        return;
      }
      return;
    }

    // Cmd/Ctrl + N: 新建
    if (e.key === "n" && e[modKey] && !e.shiftKey) {
      e.preventDefault();
      handleNewFile();
      return;
    }

    // Cmd/Ctrl + O: 打开文件
    if (e.key === "o" && e[modKey] && !e.shiftKey) {
      e.preventDefault();
      await handleOpenFile();
      return;
    }

    // Cmd/Ctrl + Shift + O: 打开文件夹
    if (e.key === "o" && e[modKey] && e.shiftKey) {
      e.preventDefault();
      await handleOpenFolder();
      return;
    }

    // Cmd/Ctrl + S: 保存
    if (e.key === "s" && e[modKey] && !e.shiftKey) {
      e.preventDefault();
      await handleSaveFile();
      return;
    }

    // Cmd/Ctrl + Shift + S: 另存为
    if (e.key === "s" && e[modKey] && e.shiftKey) {
      e.preventDefault();
      await handleSaveAs();
      return;
    }

    // Cmd/Ctrl + 1: 左右对比
    if (e.key === "1" && e[modKey]) {
      e.preventDefault();
      store.setViewMode("split");
      return;
    }

    // Cmd/Ctrl + 2: 仅编辑
    if (e.key === "2" && e[modKey]) {
      e.preventDefault();
      store.setViewMode("editor-only");
      return;
    }

    // Cmd/Ctrl + 3: 仅预览
    if (e.key === "3" && e[modKey]) {
      e.preventDefault();
      store.setViewMode("preview-only");
      return;
    }

    // Cmd/Ctrl + B: 切换侧边栏
    if (e.key === "b" && e[modKey] && !e.shiftKey) {
      e.preventDefault();
      handleToggleSidebar();
      return;
    }

    // Cmd/Ctrl + T: 切换目录
    if (e.key === "t" && e[modKey]) {
      e.preventDefault();
      handleToggleToc();
      return;
    }

    // Escape: 取消某些操作（如关闭弹窗）
    if (e.key === "Escape") {
      // 空处理，留给各组件自己管理
    }
  });
}

// 通过路径直接打开文件（用于拖拽和文件关联）
async function handleOpenFileByPath(path: string) {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const content = await invoke<string>("read_file", { path });
    const name = path.split("/").pop() || path.split("\\").pop() || "未命名.md";
    store.loadFile(path, name, content);
  } catch (e: any) {
    console.error("Open file by path error:", e);
    const msg = e?.toString() || String(e);
    if (msg.includes("permission") || msg.includes("denied") || msg.includes("not permitted")) {
      await ask(
        `无法打开文件: ${path}\n\n请前往「系统设置 → 隐私与安全性 → 文件和文件夹」授予 MD Preview 文件访问权限。`,
        { title: "权限不足", kind: "warning" }
      );
    } else {
      await ask(`无法打开文件: ${msg}`, { title: "打开失败", kind: "error" });
    }
  }
}

// 显示快捷键帮助
async function handleShowHelp() {
  try {
    const { message } = await import("@tauri-apps/plugin-dialog");
    const isMac = navigator.platform.toLowerCase().includes("mac");
    const mod = isMac ? "⌘" : "Ctrl";
    await message(
      `文件操作
  ${mod}+N  新建文件
  ${mod}+O  打开文件
  ${mod}+Shift+O  打开文件夹
  ${mod}+S  保存
  ${mod}+Shift+S  另存为

视图切换
  ${mod}+1  左右对比
  ${mod}+2  仅编辑
  ${mod}+3  仅预览

面板控制
  ${mod}+B  文件树
  ${mod}+T  目录面板
  ${mod}+Shift+B  隐藏/显示工具栏
  ${mod}+Shift+H  禅模式（一键隐藏/恢复所有面板）

其他
  ${mod}+?  显示快捷键帮助`,
      { title: "快捷键", kind: "info" }
    );
  } catch {
    // 非 Tauri 环境忽略
  }
}

// 文件操作
async function handleOpenFile() {
  const result = await file.openFile();
  if (result) {
    store.loadFile(result.path, result.name, result.content);
  }
}

async function handleSaveFile() {
  console.log("[save] editorContent length:", store.editorContent.length);
  console.log("[save] currentFile.path:", store.currentFile.path);
  const result = await file.saveFile(store.editorContent, store.currentFile.path);
  console.log("[save] result:", result);
  if (result) {
    store.markSaved(result.path, result.name);
  }
}

async function handleSaveAs() {
  const result = await file.saveAsFile(store.editorContent);
  if (result) {
    store.markSaved(result.path, result.name);
  }
}

async function handleOpenFolder() {
  const result = await file.openFolder();
  if (result) {
    store.openFolder = result.path;
    store.setFileTree(result.tree);
    showSidebar.value = true;
  }
}

async function handleOpenRecentFile(path: string) {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const content = await invoke<string>("read_file", { path });
    const name = path.split("/").pop() || path.split("\\").pop() || "未命名.md";
    store.loadFile(path, name, content);
  } catch (e: any) {
    console.error("Open recent file error:", e);
    const msg = e?.toString() || String(e);
    if (msg.includes("permission") || msg.includes("denied") || msg.includes("not permitted")) {
      await ask(
        `无法打开文件: ${path}\n\n请前往「系统设置 → 隐私与安全性 → 文件和文件夹」授予 MD Preview 文件访问权限。`,
        { title: "权限不足", kind: "warning" }
      );
    } else {
      await ask(`无法打开文件: ${msg}`, { title: "打开失败", kind: "error" });
    }
  }
}

async function handleFileSelect(filePath: string) {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const content = await invoke<string>("read_file", { path: filePath });
    const name = filePath.split("/").pop() || filePath.split("\\").pop() || "未命名.md";
    store.loadFile(filePath, name, content);
    store.selectedFile = filePath;
  } catch (e) {
    console.error("Read file error:", e);
  }
}

function handleNewFile() {
  store.newFile();
}

function handleToggleSidebar() {
  showSidebar.value = !showSidebar.value;
}

function handleToggleToc() {
  showTocPanel.value = !showTocPanel.value;
  store.updateSettings({ showToc: showTocPanel.value });
}

function handleToggleToolbar() {
  showToolbar.value = !showToolbar.value;
}

function handleRemoveRecentFile(path: string, event: Event) {
  event.stopPropagation();
  store.removeRecentFile(path);
}

function handleClearRecentFiles() {
  store.clearRecentFiles();
}

// 禅模式：一键隐藏/恢复所有面板
function handleZenMode() {
  const anyVisible = (!isMac && showToolbar.value) || showSidebar.value || showTocPanel.value;
  if (anyVisible) {
    // 进入禅模式：保存当前状态并隐藏所有面板
    zenModeBefore.value = {
      showToolbar: showToolbar.value,
      showSidebar: showSidebar.value,
      showTocPanel: showTocPanel.value,
    };
    showToolbar.value = false;
    showSidebar.value = false;
    showTocPanel.value = false;
  } else {
    // 退出禅模式：恢复之前的状态
    showToolbar.value = zenModeBefore.value.showToolbar;
    showSidebar.value = zenModeBefore.value.showSidebar;
    showTocPanel.value = zenModeBefore.value.showTocPanel;
  }
}

// macOS 系统菜单事件处理
function handleMenuAction(action: string) {
  switch (action) {
    case "menu-new":
      handleNewFile();
      break;
    case "menu-open":
      handleOpenFile();
      break;
    case "menu-open-folder":
      handleOpenFolder();
      break;
    case "menu-save":
      handleSaveFile();
      break;
    case "menu-save-as":
      handleSaveAs();
      break;
    case "menu-view-split":
      store.setViewMode("split");
      break;
    case "menu-view-editor":
      store.setViewMode("editor-only");
      break;
    case "menu-view-preview":
      store.setViewMode("preview-only");
      break;
    case "menu-sidebar":
      handleToggleSidebar();
      break;
    case "menu-toc":
      handleToggleToc();
      break;
    case "menu-zen":
      handleZenMode();
      break;
    case "menu-theme":
      theme.setTheme(theme.isDark.value ? "light" : "dark");
      break;
    case "menu-help":
      handleShowHelp();
      break;
  }
}
</script>

<template>
  <div class="app" :class="{ dark: theme.isDark.value }">
    <!-- 工具栏（Windows 专用，macOS 使用系统菜单） -->
    <Toolbar
      v-if="showToolbar && !isMac"
      :view-mode="store.viewMode"
      :is-dark="theme.isDark.value"
      :show-sidebar="showSidebar"
      :show-toc="showTocPanel"
      :recent-files="store.recentFiles"
      @new-file="handleNewFile"
      @open-file="handleOpenFile"
      @open-folder="handleOpenFolder"
      @open-recent="handleOpenRecentFile"
      @remove-recent="handleRemoveRecentFile"
      @clear-recent="handleClearRecentFiles"
      @save-file="handleSaveFile"
      @save-as="handleSaveAs"
      @toggle-sidebar="handleToggleSidebar"
      @toggle-toc="handleToggleToc"
      @toggle-toolbar="handleToggleToolbar"
      @change-view-mode="store.setViewMode"
      @toggle-theme="theme.setTheme"
    />

    <div class="main-layout">
      <!-- 左侧边栏 -->
      <aside v-if="showSidebar" class="sidebar">
        <FileTree
          :file-tree="store.fileTree"
          :open-folder="store.openFolder"
          :selected-file="store.selectedFile"
          @select-file="handleFileSelect"
        />
      </aside>

      <!-- 中间区域 -->
      <div class="content-area">
        <div class="editor-preview-wrapper">
          <!-- 仅编辑模式 -->
          <template v-if="store.viewMode === 'editor-only'">
            <EditorPane
              :content="store.editorContent"
              :font-size="store.settings.fontSize"
              :word-wrap="store.settings.wordWrap"
              :show-line-numbers="store.settings.showLineNumbers"
              @update="store.setContent"
            />
          </template>

          <!-- 仅预览模式 -->
          <template v-else-if="store.viewMode === 'preview-only'">
            <PreviewPane
              :html="markdown.renderedHtml.value"
              :preview-theme="store.settings.previewTheme"
              :is-dark="theme.isDark.value"
            />
          </template>

          <!-- 左右对比模式 -->
          <Splitpanes v-else class="splitpanes">
            <Pane min-size="20">
              <EditorPane
                :content="store.editorContent"
                :font-size="store.settings.fontSize"
                :word-wrap="store.settings.wordWrap"
                :show-line-numbers="store.settings.showLineNumbers"
                @update="store.setContent"
              />
            </Pane>
            <Pane min-size="20">
              <PreviewPane
                :html="markdown.renderedHtml.value"
                :preview-theme="store.settings.previewTheme"
                :is-dark="theme.isDark.value"
              />
            </Pane>
          </Splitpanes>
        </div>
      </div>

      <!-- 右侧目录 -->
      <aside v-if="showTocPanel && store.settings.showToc" class="toc-sidebar">
        <TocPanel :toc="markdown.toc.value" />
      </aside>
    </div>

    <StatusBar
      :file-name="store.displayTitle"
      :word-count="store.editorContent.split(/\s+/).filter(Boolean).length"
      :char-count="store.editorContent.length"
      :cursor-line="1"
      :cursor-col="1"
      :zen-mode="!showToolbar && !showSidebar && !showTocPanel"
      :view-mode="store.viewMode"
      @show-help="handleShowHelp"
      @change-view-mode="store.setViewMode"
    />
    <UpdateChecker />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app, .app {
  height: 100%;
  width: 100%;
  overflow: hidden;
}

.app {
  display: flex;
  flex-direction: column;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  background: #f5f5f5;
  color: #333;
}

.app.dark {
  background: #1e1e1e;
  color: #ccc;
}

.main-layout {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.sidebar {
  width: 220px;
  min-width: 180px;
  max-width: 320px;
  border-right: 1px solid #ddd;
  background: #fafafa;
  overflow: auto;
  display: flex;
  flex-direction: column;
}

.app.dark .sidebar {
  border-right-color: #333;
  background: #252526;
}

.content-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
}

.editor-preview-wrapper {
  flex: 1;
  overflow: hidden;
}

.splitpanes {
  height: 100%;
}

.toc-sidebar {
  width: 200px;
  min-width: 160px;
  max-width: 280px;
  border-left: 1px solid #ddd;
  background: #fafafa;
  overflow: auto;
}

.app.dark .toc-sidebar {
  border-left-color: #333;
  background: #252526;
}

/* Splitpanes 样式 */
.splitpanes__pane {
  overflow: hidden;
}

.splitpanes__splitter {
  background: #ddd;
  position: relative;
}

.app.dark .splitpanes__splitter {
  background: #333;
}

.splitpanes__splitter::before {
  content: "";
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 4px;
  height: 30px;
  background: #aaa;
  border-radius: 2px;
}

.app.dark .splitpanes__splitter::before {
  background: #555;
}
</style>
