import { ref } from "vue";
import { marked } from "marked";
import { gfmHeadingId } from "marked-gfm-heading-id";
import hljs from "highlight.js";
import "highlight.js/styles/github.css";
import katex from "katex";
import "katex/dist/katex.min.css";

let mermaidInitialized = false;

async function initMermaid() {
  if (mermaidInitialized) return;
  const mermaid = await import("mermaid");
  mermaid.default.initialize({
    startOnLoad: false,
    theme: "default",
    securityLevel: "loose",
  });
  mermaidInitialized = true;
}

marked.use(gfmHeadingId());

// 自定义 renderer 实现代码高亮
const renderer = new marked.Renderer();
renderer.code = function ({ text, lang }: { text: string; lang?: string; escaped?: boolean }) {
  if (lang && hljs.getLanguage(lang)) {
    try {
      const highlighted = hljs.highlight(text, { language: lang }).value;
      return `<pre><code class="language-${lang} hljs">${highlighted}</code></pre>`;
    } catch {
      // fallback
    }
  }
  const autoHighlighted = hljs.highlightAuto(text).value;
  return `<pre><code class="hljs">${autoHighlighted}</code></pre>`;
};

marked.use({ renderer });

/** 处理数学公式：$...$ 和 $$...$$ */
function renderMath(html: string): string {
  // 块级公式 $$...$$
  html = html.replace(
    /<p>\$\$([\s\S]*?)\$\$<\/p>/g,
    (_match, tex: string) => {
      try {
        const rendered = katex.renderToString(tex.trim(), {
          displayMode: true,
          throwOnError: false,
        });
        return `<div class="katex-block">${rendered}</div>`;
      } catch {
        return `<pre class="katex-error">${tex.trim()}</pre>`;
      }
    }
  );

  // 行内公式 $...$（但要避开已渲染的块级公式）
  html = html.replace(
    /(?<!\$)\$(?!\$)([^\$\n]+?)\$(?!\$)/g,
    (_match, tex: string) => {
      try {
        return katex.renderToString(tex.trim(), {
          displayMode: false,
          throwOnError: false,
        });
      } catch {
        return `$${tex}$`;
      }
    }
  );

  return html;
}

/** 处理 Mermaid 图表 */
async function renderMermaid(html: string): Promise<string> {
  await initMermaid();
  const mermaid = await import("mermaid");

  const tempDiv = document.createElement("div");
  tempDiv.innerHTML = html;

  const mermaidBlocks = tempDiv.querySelectorAll('pre > code.language-mermaid');
  for (let i = 0; i < mermaidBlocks.length; i++) {
    const codeEl = mermaidBlocks[i];
    const preEl = codeEl.parentElement;
    if (!preEl) continue;

    const source = codeEl.textContent || "";
    const id = `mermaid-${Date.now()}-${i}`;

    try {
      const { svg } = await mermaid.default.render(id, source);
      const wrapper = document.createElement("div");
      wrapper.className = "mermaid";
      wrapper.innerHTML = svg;
      preEl.replaceWith(wrapper);
    } catch {
      // 保持原样
    }
  }

  return tempDiv.innerHTML;
}

export function useMarkdown() {
  const renderedHtml = ref("");
  const toc = ref<{ level: number; text: string; id: string }[]>([]);

  async function render(markdown: string) {
    try {
      let html = await marked.parse(markdown);

      // 渲染数学公式
      html = renderMath(html);

      // 渲染 Mermaid 图表
      html = await renderMermaid(html);

      renderedHtml.value = html;
      toc.value = extractToc(markdown);
    } catch (e: any) {
      console.error("[Markdown render error]", e);
      renderedHtml.value = `<div class="render-error" style="padding:20px;color:#d32f2f;">
        <h3>预览渲染失败</h3>
        <p>文件内容可能包含不支持的格式或特殊字符。</p>
        <details>
          <summary>错误详情</summary>
          <pre style="background:#f5f5f5;padding:10px;overflow:auto;">${e?.toString() || "Unknown error"}</pre>
        </details>
      </div>`;
      toc.value = [];
    }
  }

  return {
    renderedHtml,
    toc,
    render,
  };
}

export function extractToc(md: string): { level: number; text: string; id: string }[] {
  const lines = md.split("\n");
  const result: { level: number; text: string; id: string }[] = [];
  const slugger = new Map<string, number>();

  for (const line of lines) {
    const match = line.match(/^(#{1,6})\s+(.+)$/);
    if (match) {
      const level = match[1].length;
      const text = match[2].trim();
      let id = text
        .toLowerCase()
        .replace(/[^\w\s-]/g, "")
        .replace(/\s+/g, "-")
        .replace(/-+/g, "-");

      const count = slugger.get(id) || 0;
      if (count > 0) {
        id = `${id}-${count}`;
      }
      slugger.set(id.replace(/-\d+$/, ""), count + 1);

      result.push({ level, text, id });
    }
  }
  return result;
}
