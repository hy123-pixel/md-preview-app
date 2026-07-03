<script setup lang="ts">
const props = defineProps<{
  toc: { level: number; text: string; id: string }[];
}>();

function scrollToHeading(id: string) {
  const previewEl = document.querySelector(".preview-content");
  if (!previewEl) return;
  const target = previewEl.querySelector(`#${id}`);
  if (target) {
    target.scrollIntoView({ behavior: "smooth", block: "start" });
  }
}
</script>

<template>
  <div class="toc-panel">
    <div class="toc-header">目录</div>
    <div class="toc-list">
      <div v-if="toc.length === 0" class="toc-empty">
        暂无目录
      </div>
      <div
        v-for="item in toc"
        :key="item.id"
        class="toc-item"
        :style="{ paddingLeft: `${(item.level - 1) * 12 + 8}px` }"
        @click="scrollToHeading(item.id)"
      >
        {{ item.text }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.toc-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.toc-header {
  padding: 8px 12px;
  font-size: 12px;
  font-weight: 600;
  color: #333;
  border-bottom: 1px solid #e0e0e0;
  flex-shrink: 0;
}

:global(.dark) .toc-header {
  color: #ddd;
  border-bottom-color: #333;
}

.toc-list {
  flex: 1;
  overflow: auto;
  padding: 4px 0;
}

.toc-empty {
  padding: 16px;
  text-align: center;
  color: #999;
  font-size: 12px;
}

.toc-item {
  padding: 4px 8px;
  font-size: 12px;
  color: #555;
  cursor: pointer;
  transition: background 0.1s, color 0.1s;
  border-radius: 4px;
  margin: 0 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

:global(.dark) .toc-item {
  color: #aaa;
}

.toc-item:hover {
  background: #e8e8e8;
  color: #007acc;
}

:global(.dark) .toc-item:hover {
  background: #333;
  color: #58a6ff;
}
</style>
