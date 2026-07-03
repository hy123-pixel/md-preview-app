<script setup lang="ts">
import { ref, computed, h, type PropType } from "vue";
import type { FileNode } from "../stores/editor";

const props = defineProps<{
  fileTree: FileNode[];
  openFolder: string | null;
  selectedFile: string | null;
}>();

const emit = defineEmits<{
  (e: "select-file", path: string): void;
}>();

const folderName = computed(() => {
  if (!props.openFolder) return "未打开文件夹";
  return props.openFolder.split("/").pop() || props.openFolder.split("\\").pop() || "文件夹";
});

function isMarkdown(name: string) {
  return name.endsWith(".md") || name.endsWith(".markdown");
}

function getFileIcon(node: FileNode): string {
  if (node.isDirectory) return "📁";
  if (isMarkdown(node.name)) return "📝";
  if (node.name.endsWith(".js") || node.name.endsWith(".ts")) return "📜";
  if (node.name.endsWith(".json")) return "📋";
  if (node.name.endsWith(".css") || node.name.endsWith(".scss")) return "🎨";
  if (node.name.endsWith(".html")) return "🌐";
  if (node.name.endsWith(".png") || node.name.endsWith(".jpg") || node.name.endsWith(".svg")) return "🖼️";
  return "📄";
}

function renderNode(node: FileNode, level: number): any {
  const isSelected = props.selectedFile === node.path;
  const paddingLeft = level * 12 + 8;

  const children = node.children && node.children.length > 0
    ? h("div", { class: "tree-children" },
        node.children.map((child) => renderNode(child, level + 1))
      )
    : null;

  return h("div", { class: "tree-node" }, [
    h(
      "div",
      {
        class: ["tree-item", { selected: isSelected }],
        style: { paddingLeft: `${paddingLeft}px` },
        onClick: () => {
          if (!node.isDirectory) {
            emit("select-file", node.path);
          }
        },
      },
      [
        h("span", { class: "tree-icon" }, getFileIcon(node)),
        h("span", { class: "tree-label" }, node.name),
      ]
    ),
    children,
  ]);
}
</script>

<template>
  <div class="file-tree">
    <div class="tree-header">
      <span class="tree-title">{{ folderName }}</span>
    </div>
    <div class="tree-content">
      <div v-if="fileTree.length === 0" class="tree-empty">
        暂无文件
      </div>
      <component
        :is="() => renderNode(node, 0)"
        v-for="node in fileTree"
        :key="node.path"
      />
    </div>
  </div>
</template>

<style>
.file-tree {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.tree-header {
  padding: 8px 12px;
  border-bottom: 1px solid #e0e0e0;
  flex-shrink: 0;
}

:global(.dark) .tree-header {
  border-bottom-color: #333;
}

.tree-title {
  font-size: 12px;
  font-weight: 600;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: block;
}

:global(.dark) .tree-title {
  color: #ddd;
}

.tree-content {
  flex: 1;
  overflow: auto;
  padding: 4px 0;
}

.tree-empty {
  padding: 16px;
  text-align: center;
  color: #999;
  font-size: 12px;
}

.tree-node {
  user-select: none;
}

.tree-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
  cursor: pointer;
  font-size: 13px;
  color: #333;
  transition: background 0.1s;
  border-radius: 4px;
  margin: 0 4px;
}

:global(.dark) .tree-item {
  color: #ccc;
}

.tree-item:hover {
  background: #e8e8e8;
}

:global(.dark) .tree-item:hover {
  background: #333;
}

.tree-item.selected {
  background: #007acc;
  color: #fff;
}

:global(.dark) .tree-item.selected {
  background: #007acc;
  color: #fff;
}

.tree-icon {
  font-size: 14px;
  flex-shrink: 0;
}

.tree-label {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tree-children {
  margin-top: 2px;
}
</style>
