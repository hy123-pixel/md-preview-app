<script setup lang="ts">
import { ref, watch } from "vue";

const props = defineProps<{
  viewMode: "split" | "editor-only" | "preview-only";
  isDark: boolean;
  showSidebar: boolean;
  showToc: boolean;
  recentFiles: string[];
}>();

const emit = defineEmits<{
  (e: "new-file"): void;
  (e: "open-file"): void;
  (e: "open-folder"): void;
  (e: "open-recent", path: string): void;
  (e: "remove-recent", path: string, event: Event): void;
  (e: "clear-recent"): void;
  (e: "save-file"): void;
  (e: "save-as"): void;
  (e: "toggle-sidebar"): void;
  (e: "toggle-toc"): void;
  (e: "toggle-toolbar"): void;
  (e: "change-view-mode", mode: "split" | "editor-only" | "preview-only"): void;
  (e: "toggle-theme", theme: "light" | "dark" | "system"): void;
}>();

const showFileMenu = ref(false);
const showViewMenu = ref(false);

const isMac = navigator.platform.toLowerCase().includes("mac");
const modKey = isMac ? "Cmd" : "Ctrl";

function closeMenus() {
  showFileMenu.value = false;
  showViewMenu.value = false;
}

function getFileName(path: string): string {
  return path.split("/").pop() || path.split("\\").pop() || path;
}

function truncatePath(path: string, maxLen: number = 40): string {
  if (path.length <= maxLen) return path;
  return "..." + path.slice(-maxLen + 3);
}
</script>

<template>
  <header class="toolbar">
    <div class="toolbar-left">
      <div class="logo">MD Preview</div>

      <!-- 文件下拉菜单 -->
      <div class="menu-dropdown" @mouseleave="closeMenus">
        <button class="btn-menu" @click="showFileMenu = !showFileMenu; showViewMenu = false">
          文件
        </button>
        <div v-if="showFileMenu" class="dropdown-panel">
          <div class="menu-item" @click="$emit('new-file'); closeMenus()">
            <span class="menu-label">新建</span>
            <span class="menu-shortcut">{{ modKey }}+N</span>
          </div>
          <div class="menu-item" @click="$emit('open-file'); closeMenus()">
            <span class="menu-label">打开文件</span>
            <span class="menu-shortcut">{{ modKey }}+O</span>
          </div>
          <div class="menu-item" @click="$emit('open-folder'); closeMenus()">
            <span class="menu-label">打开文件夹</span>
            <span class="menu-shortcut">{{ modKey }}+Shift+O</span>
          </div>
          <div class="menu-divider"></div>
          <div class="menu-item" @click="$emit('save-file'); closeMenus()">
            <span class="menu-label">保存</span>
            <span class="menu-shortcut">{{ modKey }}+S</span>
          </div>
          <div class="menu-item" @click="$emit('save-as'); closeMenus()">
            <span class="menu-label">另存为</span>
            <span class="menu-shortcut">{{ modKey }}+Shift+S</span>
          </div>

          <!-- 最近打开文件 -->
          <template v-if="recentFiles.length > 0">
            <div class="menu-divider"></div>
            <div class="menu-section-title">最近打开</div>
            <div
              v-for="path in recentFiles"
              :key="path"
              class="menu-item recent-item"
              @click="$emit('open-recent', path); closeMenus()"
              :title="path"
            >
              <span class="menu-label">
                <span class="recent-name">{{ getFileName(path) }}</span>
                <span class="recent-path">{{ truncatePath(path) }}</span>
              </span>
              <span class="remove-btn" @click="$emit('remove-recent', path, $event)">✕</span>
            </div>
            <div class="menu-item clear-recent" @click="$emit('clear-recent'); closeMenus()">
              <span class="menu-label">清除全部历史</span>
            </div>
          </template>
        </div>
      </div>

      <!-- 视图下拉菜单 -->
      <div class="menu-dropdown" @mouseleave="closeMenus">
        <button class="btn-menu" @click="showViewMenu = !showViewMenu; showFileMenu = false">
          视图
        </button>
        <div v-if="showViewMenu" class="dropdown-panel">
          <div class="menu-item" :class="{ active: viewMode === 'split' }" @click="$emit('change-view-mode', 'split'); closeMenus()">
            <span class="menu-label">左右对比</span>
            <span class="menu-shortcut">{{ modKey }}+1</span>
          </div>
          <div class="menu-item" :class="{ active: viewMode === 'editor-only' }" @click="$emit('change-view-mode', 'editor-only'); closeMenus()">
            <span class="menu-label">仅编辑</span>
            <span class="menu-shortcut">{{ modKey }}+2</span>
          </div>
          <div class="menu-item" :class="{ active: viewMode === 'preview-only' }" @click="$emit('change-view-mode', 'preview-only'); closeMenus()">
            <span class="menu-label">仅预览</span>
            <span class="menu-shortcut">{{ modKey }}+3</span>
          </div>
          <div class="menu-divider"></div>
          <div class="menu-item" :class="{ active: showSidebar }" @click="$emit('toggle-sidebar'); closeMenus()">
            <span class="menu-label">文件树</span>
            <span class="menu-shortcut">{{ modKey }}+B</span>
          </div>
          <div class="menu-item" :class="{ active: showToc }" @click="$emit('toggle-toc'); closeMenus()">
            <span class="menu-label">目录</span>
            <span class="menu-shortcut">{{ modKey }}+T</span>
          </div>
          <div class="menu-divider"></div>
          <div class="menu-item" @click="$emit('toggle-toolbar'); closeMenus()">
            <span class="menu-label">隐藏工具栏</span>
            <span class="menu-shortcut">{{ modKey }}+Shift+B</span>
          </div>
          <div class="menu-divider"></div>
          <div class="menu-item" @click="$emit('toggle-theme', isDark ? 'light' : 'dark'); closeMenus()">
            <span class="menu-label">{{ isDark ? '亮色模式' : '暗色模式' }}</span>
          </div>
        </div>
      </div>

      <div class="toolbar-separator"></div>
      <button class="btn-icon" title="新建 (Cmd+N)" @click="$emit('new-file')">
        <span class="icon">📄</span>
      </button>
      <button class="btn-icon" title="打开 (Cmd+O)" @click="$emit('open-file')">
        <span class="icon">📂</span>
      </button>
      <button class="btn-icon" title="保存 (Cmd+S)" @click="$emit('save-file')">
        <span class="icon">💾</span>
      </button>
    </div>

    <div class="toolbar-center">
      <button
        class="btn-mode"
        :class="{ active: viewMode === 'split' }"
        @click="$emit('change-view-mode', 'split')"
        title="左右对比"
      >
        ↔ 对比
      </button>
      <button
        class="btn-mode"
        :class="{ active: viewMode === 'editor-only' }"
        @click="$emit('change-view-mode', 'editor-only')"
        title="仅编辑"
      >
        📝 编辑
      </button>
      <button
        class="btn-mode"
        :class="{ active: viewMode === 'preview-only' }"
        @click="$emit('change-view-mode', 'preview-only')"
        title="仅预览"
      >
        👁 预览
      </button>
    </div>

    <div class="toolbar-right">
      <button
        class="btn-icon"
        :class="{ active: showSidebar }"
        @click="$emit('toggle-sidebar')"
        title="文件树"
      >
        🗂️
      </button>
      <button
        class="btn-icon"
        :class="{ active: showToc }"
        @click="$emit('toggle-toc')"
        title="目录"
      >
        📋
      </button>
      <button
        class="btn-icon"
        @click="$emit('toggle-toolbar')"
        title="隐藏工具栏"
      >
        ⬆️
      </button>
      <button
        class="btn-icon"
        @click="$emit('toggle-theme', isDark ? 'light' : 'dark')"
        :title="isDark ? '亮色' : '暗色'"
      >
        {{ isDark ? '☀️' : '🌙' }}
      </button>
    </div>
  </header>
</template>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 42px;
  padding: 0 12px;
  background: #f0f0f0;
  border-bottom: 1px solid #ddd;
  flex-shrink: 0;
  user-select: none;
}

:global(.dark) .toolbar {
  background: #2d2d2d;
  border-bottom-color: #444;
}

.toolbar-left {
  display: flex;
  align-items: center;
  gap: 4px;
}

.logo {
  font-weight: 700;
  font-size: 14px;
  color: #333;
  margin-right: 8px;
}

:global(.dark) .logo {
  color: #ddd;
}

.toolbar-separator {
  width: 1px;
  height: 20px;
  background: #ccc;
  margin-right: 4px;
}

:global(.dark) .toolbar-separator {
  background: #555;
}

.btn-icon {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: #555;
  font-size: 12px;
  cursor: pointer;
  transition: background 0.15s;
}

:global(.dark) .btn-icon {
  color: #bbb;
}

.btn-icon:hover {
  background: #e0e0e0;
}

:global(.dark) .btn-icon:hover {
  background: #3d3d3d;
}

.btn-icon.active {
  background: #d0d0d0;
}

:global(.dark) .btn-icon.active {
  background: #444;
}

.icon {
  font-size: 14px;
}

.toolbar-center {
  display: flex;
  align-items: center;
  gap: 2px;
}

.btn-mode {
  padding: 4px 10px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: #666;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.15s;
}

:global(.dark) .btn-mode {
  color: #aaa;
}

.btn-mode:hover {
  background: #e0e0e0;
}

:global(.dark) .btn-mode:hover {
  background: #3d3d3d;
}

.btn-mode.active {
  background: #ddd;
  color: #333;
  font-weight: 600;
}

:global(.dark) .btn-mode.active {
  background: #444;
  color: #fff;
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 4px;
}

/* 下拉菜单 */
.menu-dropdown {
  position: relative;
}

.btn-menu {
  padding: 4px 10px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: #555;
  font-size: 13px;
  cursor: pointer;
  transition: background 0.15s;
}

:global(.dark) .btn-menu {
  color: #bbb;
}

.btn-menu:hover {
  background: #e0e0e0;
}

:global(.dark) .btn-menu:hover {
  background: #3d3d3d;
}

.dropdown-panel {
  position: absolute;
  top: 100%;
  left: 0;
  margin-top: 4px;
  min-width: 280px;
  max-width: 400px;
  max-height: 400px;
  overflow-y: auto;
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  padding: 4px 0;
}

:global(.dark) .dropdown-panel {
  background: #2d2d2d;
  border-color: #444;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
}

.menu-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 12px;
  cursor: pointer;
  font-size: 13px;
  color: #333;
  transition: background 0.1s;
}

:global(.dark) .menu-item {
  color: #ddd;
}

.menu-item:hover {
  background: #f0f0f0;
}

:global(.dark) .menu-item:hover {
  background: #3d3d3d;
}

.menu-item.active {
  background: #e6f3ff;
  color: #007acc;
}

:global(.dark) .menu-item.active {
  background: #1a3a5c;
  color: #58a6ff;
}

.menu-label {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
}

.menu-shortcut {
  font-size: 11px;
  color: #999;
  margin-left: 12px;
  white-space: nowrap;
  flex-shrink: 0;
}

:global(.dark) .menu-shortcut {
  color: #777;
}

.menu-divider {
  height: 1px;
  background: #eee;
  margin: 4px 0;
}

:global(.dark) .menu-divider {
  background: #444;
}

.menu-section-title {
  padding: 4px 12px;
  font-size: 11px;
  font-weight: 600;
  color: #999;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  user-select: none;
}

:global(.dark) .menu-section-title {
  color: #777;
}

.recent-item {
  padding: 5px 12px;
}

.recent-name {
  font-size: 13px;
  color: #333;
  display: block;
}

:global(.dark) .recent-name {
  color: #ddd;
}

.recent-path {
  font-size: 11px;
  color: #999;
  display: block;
  margin-top: 1px;
}

:global(.dark) .recent-path {
  color: #777;
}

.remove-btn {
  font-size: 11px;
  color: #999;
  padding: 2px 6px;
  border-radius: 3px;
  cursor: pointer;
  flex-shrink: 0;
  opacity: 0;
  transition: opacity 0.15s, background 0.15s;
}

.recent-item:hover .remove-btn {
  opacity: 1;
}

.remove-btn:hover {
  background: #ff4444;
  color: #fff;
}

.clear-recent {
  padding: 4px 12px;
  font-size: 12px;
  color: #999;
}

:global(.dark) .clear-recent {
  color: #777;
}

.clear-recent:hover {
  color: #ff4444;
}

:global(.dark) .clear-recent:hover {
  color: #ff6666;
}
</style>
