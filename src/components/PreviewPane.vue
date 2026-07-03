<script setup lang="ts">
import { ref, watch, onMounted } from "vue";

const props = defineProps<{
  html: string;
  previewTheme: string;
  isDark: boolean;
}>();

const previewRef = ref<HTMLDivElement | null>(null);

onMounted(() => {
  if (previewRef.value && props.html) {
    previewRef.value.innerHTML = props.html;
  }
});

watch(() => props.html, (newVal) => {
  if (previewRef.value) {
    previewRef.value.innerHTML = newVal;
  }
});
</script>

<template>
  <div class="preview-pane" :class="[previewTheme, { dark: isDark }]">
    <div ref="previewRef" class="preview-content markdown-body"></div>
  </div>
</template>

<style scoped>
.preview-pane {
  height: 100%;
  width: 100%;
  overflow: auto;
  background: #fff;
}

.preview-pane.dark {
  background: #0d1117;
}

.preview-content {
  max-width: 900px;
  margin: 0 auto;
  padding: 32px 40px;
  min-height: 100%;
}

@media (max-width: 768px) {
  .preview-content {
    padding: 16px 20px;
  }
}
</style>

<style>
/* 基础 Markdown 样式 */
.markdown-body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Helvetica Neue", Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 1.7;
  color: #24292f;
  word-wrap: break-word;
}

.markdown-body.dark {
  color: #c9d1d9;
}

.markdown-body h1,
.markdown-body h2,
.markdown-body h3,
.markdown-body h4,
.markdown-body h5,
.markdown-body h6 {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
  line-height: 1.25;
}

.markdown-body h1 { font-size: 2em; border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
.markdown-body h2 { font-size: 1.5em; border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
.markdown-body h3 { font-size: 1.25em; }
.markdown-body h4 { font-size: 1em; }
.markdown-body h5 { font-size: 0.875em; }
.markdown-body h6 { font-size: 0.85em; color: #57606a; }

.markdown-body.dark h1,
.markdown-body.dark h2 { border-bottom-color: #30363d; }
.markdown-body.dark h6 { color: #8b949e; }

.markdown-body p {
  margin-top: 0;
  margin-bottom: 16px;
}

.markdown-body a {
  color: #0969da;
  text-decoration: none;
}

.markdown-body.dark a { color: #58a6ff; }
.markdown-body a:hover { text-decoration: underline; }

.markdown-body ul,
.markdown-body ol {
  margin-top: 0;
  margin-bottom: 16px;
  padding-left: 2em;
}

.markdown-body li + li { margin-top: 0.25em; }
.markdown-body li > p { margin-top: 16px; }

.markdown-body blockquote {
  margin: 0 0 16px;
  padding: 0 1em;
  color: #57606a;
  border-left: 0.25em solid #d0d7de;
}

.markdown-body.dark blockquote {
  color: #8b949e;
  border-left-color: #30363d;
}

.markdown-body pre {
  padding: 16px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background: #f6f8fa;
  border-radius: 6px;
  margin-bottom: 16px;
}

.markdown-body.dark pre {
  background: #161b22;
}

.markdown-body code {
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 85%;
  background: rgba(175, 184, 193, 0.2);
  border-radius: 6px;
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, Courier, monospace;
}

.markdown-body.dark code {
  background: rgba(110, 118, 129, 0.4);
}

.markdown-body pre > code {
  padding: 0;
  background: transparent;
  border-radius: 0;
  word-break: normal;
  white-space: pre;
  display: block;
}

.markdown-body hr {
  height: 0.25em;
  padding: 0;
  margin: 24px 0;
  background: #d0d7de;
  border: 0;
}

.markdown-body.dark hr { background: #30363d; }

.markdown-body table {
  border-spacing: 0;
  border-collapse: collapse;
  margin-bottom: 16px;
  width: 100%;
  overflow: auto;
  display: block;
}

.markdown-body table th,
.markdown-body table td {
  padding: 6px 13px;
  border: 1px solid #d0d7de;
}

.markdown-body.dark table th,
.markdown-body.dark table td { border-color: #30363d; }

.markdown-body table th {
  font-weight: 600;
  background: #f6f8fa;
}

.markdown-body.dark table th { background: #161b22; }

.markdown-body table tr:nth-child(2n) { background: #f6f8fa; }
.markdown-body.dark table tr:nth-child(2n) { background: #161b22; }

.markdown-body img {
  max-width: 100%;
  box-sizing: border-box;
  background: #fff;
}

.markdown-body input[type="checkbox"] {
  margin-right: 0.5em;
}

.markdown-body .mermaid {
  text-align: center;
  margin: 16px 0;
}

.markdown-body .katex-block {
  overflow-x: auto;
  margin: 16px 0;
}

.markdown-body .katex-error {
  color: #cf222e;
  background: #ffebe9;
  padding: 8px;
  border-radius: 4px;
}
</style>
