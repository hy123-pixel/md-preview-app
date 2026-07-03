<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue";
import { EditorView, keymap, lineNumbers, highlightActiveLine, highlightActiveLineGutter } from "@codemirror/view";
import { EditorState, Compartment } from "@codemirror/state";
import { markdown } from "@codemirror/lang-markdown";
import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import { searchKeymap, highlightSelectionMatches } from "@codemirror/search";
import { oneDark } from "@codemirror/theme-one-dark";
import { syntaxHighlighting, defaultHighlightStyle, indentOnInput } from "@codemirror/language";
import { closeBrackets, closeBracketsKeymap } from "@codemirror/autocomplete";

const props = defineProps<{
  content: string;
  fontSize: number;
  wordWrap: boolean;
  showLineNumbers: boolean;
}>();

const emit = defineEmits<{
  (e: "update", value: string): void;
}>();

const editorRef = ref<HTMLDivElement | null>(null);
let editorView: EditorView | null = null;
const themeCompartment = new Compartment();
const lineNumberCompartment = new Compartment();
const wrapCompartment = new Compartment();

function getThemeExtension() {
  return document.documentElement.classList.contains("dark") ? oneDark : [];
}

function getLineNumberExtension() {
  return props.showLineNumbers ? [lineNumbers(), highlightActiveLineGutter()] : [];
}

function getWrapExtension() {
  return props.wordWrap ? EditorView.lineWrapping : [];
}

function createEditor() {
  if (!editorRef.value) return;

  const state = EditorState.create({
    doc: props.content,
    extensions: [
      history(),
      markdown(),
      indentOnInput(),
      closeBrackets(),
      highlightActiveLine(),
      highlightSelectionMatches(),
      syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
      keymap.of([
        ...closeBracketsKeymap,
        ...defaultKeymap,
        ...searchKeymap,
        ...historyKeymap,
        indentWithTab,
      ]),
      themeCompartment.of(getThemeExtension()),
      lineNumberCompartment.of(getLineNumberExtension()),
      wrapCompartment.of(getWrapExtension()),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          emit("update", update.state.doc.toString());
        }
      }),
      EditorView.theme({
        "&": { fontSize: `${props.fontSize}px` },
        ".cm-content": { fontFamily: "'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace" },
      }),
    ],
  });

  editorView = new EditorView({
    state,
    parent: editorRef.value,
  });
}

onMounted(() => {
  createEditor();

  // 监听暗色模式变化
  const observer = new MutationObserver(() => {
    if (editorView) {
      editorView.dispatch({
        effects: themeCompartment.reconfigure(getThemeExtension()),
      });
    }
  });
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ["class"] });

  onUnmounted(() => {
    observer.disconnect();
    if (editorView) {
      editorView.destroy();
      editorView = null;
    }
  });
});

// 外部内容变化时同步（如打开新文件）
watch(() => props.content, (newVal) => {
  if (editorView && editorView.state.doc.toString() !== newVal) {
    const transaction = editorView.state.update({
      changes: { from: 0, to: editorView.state.doc.length, insert: newVal },
    });
    editorView.dispatch(transaction);
  }
});

watch(() => props.fontSize, (newSize) => {
  if (editorView) {
    const themeExt = EditorView.theme({
      "&": { fontSize: `${newSize}px` },
    });
    editorView.dispatch({
      effects: themeCompartment.reconfigure(themeExt),
    });
  }
});

watch(() => props.showLineNumbers, () => {
  if (editorView) {
    editorView.dispatch({
      effects: lineNumberCompartment.reconfigure(getLineNumberExtension()),
    });
  }
});

watch(() => props.wordWrap, () => {
  if (editorView) {
    editorView.dispatch({
      effects: wrapCompartment.reconfigure(getWrapExtension()),
    });
  }
});
</script>

<template>
  <div class="editor-pane">
    <div ref="editorRef" class="editor-container"></div>
  </div>
</template>

<style scoped>
.editor-pane {
  height: 100%;
  width: 100%;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  background: #fff;
}

:global(.dark) .editor-pane {
  background: #1e1e1e;
}

.editor-container {
  flex: 1;
  overflow: hidden;
}

.editor-container :deep(.cm-editor) {
  height: 100%;
}

.editor-container :deep(.cm-scroller) {
  font-family: "SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, "Courier New", monospace;
  line-height: 1.6;
}

.editor-container :deep(.cm-content) {
  padding: 12px 0;
}

.editor-container :deep(.cm-line) {
  padding: 0 12px 0 4px;
}
</style>
