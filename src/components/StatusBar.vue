<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  fileName: string;
  wordCount: number;
  charCount: number;
  cursorLine: number;
  cursorCol: number;
  zenMode?: boolean;
  viewMode?: "split" | "editor-only" | "preview-only";
}>();

const emit = defineEmits<{
  (e: "show-help"): void;
  (e: "change-view-mode", mode: "split" | "editor-only" | "preview-only"): void;
}>();

const isMac = computed(() => navigator.platform.toLowerCase().includes("mac"));
const osName = computed(() => (isMac.value ? "macOS" : "Windows"));
const exitZenHint = computed(() => (isMac.value ? "⌘⇧H 退出" : "Ctrl+Shift+H 退出"));
</script>

<template>
  <footer class="status-bar">
    <div class="status-left">
      <span class="status-item" :title="fileName">{{ fileName }}</span>
      <span v-if="zenMode" class="zen-badge" :title="`禅模式 — ${exitZenHint}`">🧘 禅模式</span>
    </div>
    <div class="status-right">
      <!-- 视图快速切换 -->
      <div class="view-switcher">
        <button
          class="view-btn"
          :class="{ active: viewMode === 'split' }"
          title="左右对比"
          @click="$emit('change-view-mode', 'split')"
        >
          对比
        </button>
        <button
          class="view-btn"
          :class="{ active: viewMode === 'editor-only' }"
          title="仅编辑"
          @click="$emit('change-view-mode', 'editor-only')"
        >
          编辑
        </button>
        <button
          class="view-btn"
          :class="{ active: viewMode === 'preview-only' }"
          title="仅预览"
          @click="$emit('change-view-mode', 'preview-only')"
        >
          预览
        </button>
      </div>
      <span v-if="zenMode" class="zen-hint">{{ exitZenHint }}</span>
      <span class="status-item">{{ wordCount }} 词</span>
      <span class="status-item">{{ charCount }} 字符</span>
      <span class="status-item">Ln {{ cursorLine }}, Col {{ cursorCol }}</span>
      <span class="status-item">{{ osName }}</span>
      <button class="help-btn" :title="isMac ? '快捷键帮助 (⌘+?)' : '快捷键帮助 (Ctrl+?)'" @click="$emit('show-help')">
        ?
      </button>
    </div>
  </footer>
</template>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 24px;
  padding: 0 12px;
  background: #f0f0f0;
  border-top: 1px solid #ddd;
  font-size: 12px;
  color: #666;
  flex-shrink: 0;
  user-select: none;
}

:global(.dark) .status-bar {
  background: #2d2d2d;
  border-top-color: #444;
  color: #aaa;
}

.status-left {
  display: flex;
  align-items: center;
  gap: 12px;
  overflow: hidden;
}

.status-right {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.status-item {
  white-space: nowrap;
}

.zen-badge {
  font-size: 11px;
  color: #4a90d9;
  margin-left: 8px;
  cursor: default;
}

:global(.dark) .zen-badge {
  color: #6ab0ff;
}

.zen-hint {
  font-size: 11px;
  color: #999;
  white-space: nowrap;
}

:global(.dark) .zen-hint {
  color: #777;
}

/* 视图快速切换 */
.view-switcher {
  display: flex;
  align-items: center;
  gap: 1px;
  border: 1px solid #ccc;
  border-radius: 4px;
  overflow: hidden;
  background: #e8e8e8;
}

:global(.dark) .view-switcher {
  border-color: #555;
  background: #3a3a3a;
}

.view-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 36px;
  height: 20px;
  border: none;
  background: transparent;
  color: #888;
  font-size: 11px;
  cursor: pointer;
  padding: 0 6px;
  line-height: 1;
  transition: background 0.15s, color 0.15s;
  white-space: nowrap;
}

:global(.dark) .view-btn {
  color: #aaa;
}

.view-btn:hover {
  background: #d0d0d0;
  color: #333;
}

:global(.dark) .view-btn:hover {
  background: #444;
  color: #fff;
}

.view-btn.active {
  background: #fff;
  color: #333;
  font-weight: 600;
}

:global(.dark) .view-btn.active {
  background: #555;
  color: #fff;
}

.help-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: 1px solid #ccc;
  background: transparent;
  color: #999;
  font-size: 11px;
  font-weight: 600;
  cursor: pointer;
  padding: 0;
  line-height: 1;
  transition: all 0.15s;
}

:global(.dark) .help-btn {
  border-color: #555;
  color: #aaa;
}

.help-btn:hover {
  background: #e0e0e0;
  color: #333;
}

:global(.dark) .help-btn:hover {
  background: #444;
  color: #fff;
}

.status-left .status-item {
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 300px;
}
</style>
